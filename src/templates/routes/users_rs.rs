use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let db_queries = match config.database {
        crate::config::Database::MongoDb => {
            r#"
async fn get_all_users(pool: &DbPool) -> Result<Vec<User>> {
    use mongodb::bson::doc;
    use futures::stream::TryStreamExt;

    let collection = pool.database().collection::<User>("users");
    let cursor = collection
        .find(None, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    cursor
        .try_collect()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn get_user_by_id(pool: &DbPool, user_id: &str) -> Result<Option<User>> {
    use mongodb::bson::doc;

    let collection = pool.database().collection::<User>("users");
    collection
        .find_one(doc! { "id": user_id }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn delete_user_by_id(pool: &DbPool, user_id: &str) -> Result<bool> {
    use mongodb::bson::doc;

    let collection = pool.database().collection::<User>("users");
    let result = collection
        .delete_one(doc! { "id": user_id }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(result.deleted_count > 0)
}
"#
        }
        _ => {
            r#"
async fn get_all_users(pool: &DbPool) -> Result<Vec<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn get_user_by_id(pool: &DbPool, user_id: String) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn delete_user_by_id(pool: &DbPool, user_id: String) -> Result<bool> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(result.rows_affected() > 0)
}
"#
        }
    };

    let get_user_handler = if config.database == crate::config::Database::MongoDb {
        r#"
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>> {
    let user = get_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}
"#
    } else {
        r#"
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>> {
    let user = get_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}
"#
    };

    let delete_user_handler = if config.database == crate::config::Database::MongoDb {
        r#"
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<StatusCode> {
    let deleted = delete_user_by_id(&state.pool, user_id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("User not found".to_string()))
    }
}
"#
    } else {
        r#"
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<StatusCode> {
    let deleted = delete_user_by_id(&state.pool, user_id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("User not found".to_string()))
    }
}
"#
    };

    format!(
        r#"use axum::{{
    extract::{{Path, State}},
    http::StatusCode,
    Extension, Json,
}};

use crate::{{
    app_state::AppState,
    db::DbPool,
    error::{{AppError, Result}},
    models::{{User, UserResponse}},
}};
{}
pub async fn get_current_user(
    Extension(user): Extension<User>,
) -> Result<Json<UserResponse>> {{
    Ok(Json(UserResponse::from(user)))
}}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>> {{
    let users = get_all_users(&state.pool).await?;
    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(responses))
}}
{}{}
"#,
        db_queries, get_user_handler, delete_user_handler
    )
}
