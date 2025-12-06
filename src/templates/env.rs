use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let jwt_config = if config.auth_method.supports_jwt() {
        r#"
# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION_HOURS=24
"#
    } else {
        ""
    };

    let session_config = if config.auth_method.supports_session() {
        r#"
# Session Configuration
REDIS_URL=redis://127.0.0.1:6379
SESSION_EXPIRATION_HOURS=168
"#
    } else {
        ""
    };

    let oauth_config = if config.include_google_oauth {
        r#"
# Google OAuth Configuration
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback
"#
    } else {
        ""
    };

    format!(
        r#"# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Database Configuration
DATABASE_URL={}
{}{}{}
# Logging
RUST_LOG=debug,tower_http=debug
"#,
        config.database.connection_string_template(),
        jwt_config,
        session_config,
        oauth_config
    )
}
