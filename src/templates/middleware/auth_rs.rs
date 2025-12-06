use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let jwt_extract = if config.auth_method.supports_jwt() {
        r#"
    if let Some(auth_header) = headers.get(axum::http::header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match crate::auth::verify_jwt(token, &state.config.jwt_secret) {
                    Ok(claims) => {
                        let user_id = claims.sub.clone();
                        return match get_user_by_id(&state.pool, user_id).await {
                            Ok(Some(user)) => {
                                request.extensions_mut().insert(user);
                                Ok(next.run(request).await)
                            }
                            Ok(None) => Err(AppError::AuthenticationError("User not found".to_string())),
                            Err(e) => Err(e),
                        };
                    }
                    Err(_) => {}
                }
            }
        }
    }
"#
    } else {
        ""
    };

    let session_extract = if config.auth_method.supports_session() {
        r#"
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(session_token) = cookie.strip_prefix("session_token=") {
                    if let Ok(user_id) = state.session_store.get_user_id(session_token).await {
                        return match get_user_by_id(&state.pool, user_id).await {
                            Ok(Some(user)) => {
                                request.extensions_mut().insert(user);
                                Ok(next.run(request).await)
                            }
                            Ok(None) => Err(AppError::AuthenticationError("User not found".to_string())),
                            Err(e) => Err(e),
                        };
                    }
                }
            }
        }
    }
"#
    } else {
        ""
    };

    let get_user_fn = match config.database {
        crate::config::Database::MongoDb => {
            r#"
async fn get_user_by_id(pool: &DbPool, user_id: String) -> Result<Option<User>> {
    use mongodb::bson::doc;

    let collection = pool.database().collection::<User>("users");
    let user = collection
        .find_one(doc! { "id": &user_id }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(user)
}
"#
        }
        _ => {
            r#"
async fn get_user_by_id(pool: &DbPool, user_id: String) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
"#
        }
    };

    format!(
        r#"use axum::{{
    extract::{{Request, State}},
    middleware::Next,
    response::Response,
}};

use crate::{{
    app_state::AppState,
    db::DbPool,
    error::{{AppError, Result}},
    models::User,
}};
{}
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response> {{
    let headers = request.headers();
    {}{}
    Err(AppError::AuthenticationError("Authentication required".to_string()))
}}
"#,
        get_user_fn, jwt_extract, session_extract
    )
}
