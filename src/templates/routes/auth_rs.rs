use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let jwt_response = if config.auth_method.supports_jwt() {
        r#"let token = crate::auth::create_jwt(
            user.id,
            user.email.clone(),
            user.role.clone(),
            &state.config.jwt_secret,
            state.config.jwt_expiration_hours,
        )?;
        response["token"] = serde_json::Value::String(token);"#
    } else {
        ""
    };

    let session_response = if config.auth_method.supports_session() {
        r#"let session_token = state.session_store.create_session(user.id).await?;
        let cookie = format!(
            "session_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
            session_token,
            state.config.session_expiration_hours * 3600
        );
        headers.insert(
            axum::http::header::SET_COOKIE,
            cookie.parse().unwrap(),
        );"#
    } else {
        ""
    };

    let create_user_fn = match config.database {
        crate::config::Database::MongoDb => {
            r#"
async fn create_user(pool: &DbPool, dto: CreateUserDto, password_hash: String) -> Result<User> {
    let user = User {
        id: None,
        email: dto.email,
        password_hash: Some(password_hash),
        name: dto.name,
        role: UserRole::User.as_str().to_string(),
        google_id: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let collection = pool.database().collection::<User>("users");
    collection
        .insert_one(&user, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(user)
}

async fn get_user_by_email(pool: &DbPool, email: &str) -> Result<Option<User>> {
    use mongodb::bson::doc;

    let collection = pool.database().collection::<User>("users");
    collection
        .find_one(doc! { "email": email }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
"#
        }
        _ => {
            r#"
async fn create_user(pool: &DbPool, dto: CreateUserDto, password_hash: String) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name, role) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&dto.email)
    .bind(&password_hash)
    .bind(&dto.name)
    .bind(UserRole::User.as_str())
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn get_user_by_email(pool: &DbPool, email: &str) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
"#
        }
    };

    let oauth_helpers = if config.include_google_oauth {
        match config.database {
            crate::config::Database::MongoDb => {
                r#"
async fn update_user_google_id(pool: &DbPool, email: &str, google_id: &str) -> Result<()> {
    use mongodb::bson::doc;

    let collection = pool.database().collection::<User>("users");
    collection
        .update_one(
            doc! { "email": email },
            doc! { "$set": { "google_id": google_id } },
            None
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

async fn create_oauth_user(pool: &DbPool, user_info: crate::auth::GoogleUserInfo) -> Result<User> {
    let user = User {
        id: None,
        email: user_info.email,
        password_hash: None,
        name: user_info.name,
        role: UserRole::User.as_str().to_string(),
        google_id: Some(user_info.id),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let collection = pool.database().collection::<User>("users");
    collection
        .insert_one(&user, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(user)
}
"#
            }
            _ => {
                r#"
async fn update_user_google_id(pool: &DbPool, email: &str, google_id: &str) -> Result<()> {
    sqlx::query("UPDATE users SET google_id = $1 WHERE email = $2")
        .bind(google_id)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

async fn create_oauth_user(pool: &DbPool, user_info: crate::auth::GoogleUserInfo) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, name, role, google_id) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&user_info.email)
    .bind(&user_info.name)
    .bind(UserRole::User.as_str())
    .bind(&user_info.id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}
"#
            }
        }
    } else {
        ""
    };

    let oauth_routes = if config.include_google_oauth {
        r#"
pub async fn google_login(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>> {
    let oauth_client = state
        .oauth_client
        .as_ref()
        .as_ref()
        .ok_or_else(|| AppError::InternalError("OAuth not configured".to_string()))?;

    let (auth_url, _csrf_token) = oauth_client.get_authorization_url();

    Ok(Json(json!({
        "url": auth_url
    })))
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<(StatusCode, HeaderMap, Json<serde_json::Value>)> {
    let oauth_client = state
        .oauth_client
        .as_ref()
        .as_ref()
        .ok_or_else(|| AppError::InternalError("OAuth not configured".to_string()))?;

    let code = params
        .get("code")
        .ok_or_else(|| AppError::ValidationError("Missing code parameter".to_string()))?;

    let user_info = oauth_client.exchange_code(code.to_string()).await?;

    let existing_user = get_user_by_email(&state.pool, &user_info.email).await?;

    let user = if let Some(mut user) = existing_user {
        if user.google_id.is_none() {
            user.google_id = Some(user_info.id.clone());
            update_user_google_id(&state.pool, &user.email, &user_info.id).await?;
        }
        user
    } else {
        create_oauth_user(&state.pool, user_info).await?
    };

    let mut response = json!({
        "message": "Login successful",
        "user": UserResponse::from(user.clone())
    });

    let mut headers = HeaderMap::new();

    Ok((StatusCode::OK, headers, Json(response)))
}
"#
    } else {
        ""
    };

    format!(
        r#"use axum::{{
    extract::{{Query, State}},
    http::{{HeaderMap, StatusCode}},
    Json,
}};
use serde::Deserialize;
use serde_json::json;

use crate::{{
    app_state::AppState,
    auth::{{hash_password, verify_password}},
    db::DbPool,
    error::{{AppError, Result}},
    models::{{CreateUserDto, User, UserResponse, UserRole}},
}};

#[derive(Debug, Deserialize)]
pub struct LoginDto {{
    pub email: String,
    pub password: String,
}}

pub async fn signup(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> Result<(StatusCode, HeaderMap, Json<serde_json::Value>)> {{
    let existing_user = get_user_by_email(&state.pool, &dto.email).await?;
    if existing_user.is_some() {{
        return Err(AppError::ValidationError("Email already exists".to_string()));
    }}

    let password_hash = hash_password(&dto.password)?;
    let user = create_user(&state.pool, dto, password_hash).await?;

    let mut response = json!({{
        "message": "User created successfully",
        "user": UserResponse::from(user.clone())
    }});

    let mut headers = HeaderMap::new();
    {}{}
    Ok((StatusCode::CREATED, headers, Json(response)))
}}

pub async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> Result<(StatusCode, HeaderMap, Json<serde_json::Value>)> {{
    let user = get_user_by_email(&state.pool, &dto.email)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::AuthenticationError("Password login not available for this account".to_string()))?;

    if !verify_password(&dto.password, password_hash)? {{
        return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
    }}

    let mut response = json!({{
        "message": "Login successful",
        "user": UserResponse::from(user.clone())
    }});

    let mut headers = HeaderMap::new();
    {}{}
    Ok((StatusCode::OK, headers, Json(response)))
}}
{}
{}
{}
"#,
        jwt_response,
        session_response,
        jwt_response,
        session_response,
        oauth_routes,
        create_user_fn,
        oauth_helpers
    )
}
