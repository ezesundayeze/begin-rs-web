use crate::config::ProjectConfig;

pub fn generate(_config: &ProjectConfig) -> &'static str {
    r#"use chrono::{Duration, Utc};
use redis::{AsyncCommands, Client};
use uuid::Uuid;

use crate::error::{AppError, Result};

pub struct SessionStore {
    client: Client,
    expiration_hours: i64,
}

impl SessionStore {
    pub async fn new(redis_url: &str, expiration_hours: i64) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?;

        Ok(Self {
            client,
            expiration_hours,
        })
    }

    pub async fn create_session(&self, user_id: String) -> Result<String> {
        let session_token = Uuid::new_v4().to_string();
        let key = format!("session:{}", session_token);

        let mut conn = self.client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection error: {}", e)))?;

        let expiration_seconds = self.expiration_hours * 3600;

        conn.set_ex(&key, user_id, expiration_seconds as u64)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to create session: {}", e)))?;

        Ok(session_token)
    }

    pub async fn get_user_id(&self, session_token: &str) -> Result<String> {
        let key = format!("session:{}", session_token);

        let mut conn = self.client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection error: {}", e)))?;

        let user_id: Option<String> = conn.get(&key).await
            .map_err(|e| AppError::InternalError(format!("Failed to get session: {}", e)))?;

        user_id.ok_or_else(|| AppError::AuthenticationError("Invalid session".to_string()))
    }

    pub async fn delete_session(&self, session_token: &str) -> Result<()> {
        let key = format!("session:{}", session_token);

        let mut conn = self.client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection error: {}", e)))?;

        conn.del(&key).await
            .map_err(|e| AppError::InternalError(format!("Failed to delete session: {}", e)))?;

        Ok(())
    }

    pub async fn refresh_session(&self, session_token: &str) -> Result<()> {
        let key = format!("session:{}", session_token);

        let mut conn = self.client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection error: {}", e)))?;

        let expiration_seconds = self.expiration_hours * 3600;

        conn.expire(&key, expiration_seconds as i64).await
            .map_err(|e| AppError::InternalError(format!("Failed to refresh session: {}", e)))?;

        Ok(())
    }
}
"#
}
