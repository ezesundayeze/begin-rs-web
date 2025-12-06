use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let session_field = if config.auth_method.supports_session() {
        "    pub session_store: Arc<SessionStore>,"
    } else {
        ""
    };

    let oauth_field = if config.include_google_oauth {
        "    pub oauth_client: Arc<Option<GoogleOAuthClient>>,"
    } else {
        ""
    };

    let session_import = if config.auth_method.supports_session() {
        "use crate::auth::SessionStore;"
    } else {
        ""
    };

    let oauth_import = if config.include_google_oauth {
        "use crate::auth::GoogleOAuthClient;"
    } else {
        ""
    };

    format!(
        r#"use std::sync::Arc;
use crate::{{
    config::Config,
    db::DbPool,
}};
{}
{}

#[derive(Clone)]
pub struct AppState {{
    pub pool: DbPool,
    pub config: Arc<Config>,
{}
{}
}}
"#,
        session_import, oauth_import, session_field, oauth_field
    )
}
