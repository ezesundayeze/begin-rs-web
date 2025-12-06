use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let jwt_config = if config.auth_method.supports_jwt() {
        r#"
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,"#
    } else {
        ""
    };

    let session_config = if config.auth_method.supports_session() {
        r#"
    pub redis_url: String,
    pub session_expiration_hours: i64,"#
    } else {
        ""
    };

    let oauth_config = if config.include_google_oauth {
        r#"
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_uri: Option<String>,"#
    } else {
        ""
    };

    let jwt_load = if config.auth_method.supports_jwt() {
        r#"
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a number"),"#
    } else {
        ""
    };

    let session_load = if config.auth_method.supports_session() {
        r#"
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            session_expiration_hours: std::env::var("SESSION_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "168".to_string())
                .parse()
                .expect("SESSION_EXPIRATION_HOURS must be a number"),"#
    } else {
        ""
    };

    let oauth_load = if config.include_google_oauth {
        r#"
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            google_redirect_uri: std::env::var("GOOGLE_REDIRECT_URI").ok(),"#
    } else {
        ""
    };

    format!(
        r#"use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {{
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,{}{}{}
}}

impl Config {{
    pub fn from_env() -> Result<Self> {{
        dotenvy::dotenv().ok();

        Ok(Self {{
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),{}{}{}
        }})
    }}

    pub fn server_address(&self) -> String {{
        format!("{{}}:{{}}", self.server_host, self.server_port)
    }}
}}
"#,
        jwt_config, session_config, oauth_config, jwt_load, session_load, oauth_load
    )
}
