use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let jwt_mod = if config.auth_method.supports_jwt() {
        "pub mod jwt;"
    } else {
        ""
    };

    let session_mod = if config.auth_method.supports_session() {
        "pub mod session;"
    } else {
        ""
    };

    let oauth_mod = if config.include_google_oauth {
        "pub mod oauth;"
    } else {
        ""
    };

    let jwt_pub = if config.auth_method.supports_jwt() {
        "pub use jwt::*;"
    } else {
        ""
    };

    let session_pub = if config.auth_method.supports_session() {
        "pub use session::*;"
    } else {
        ""
    };

    let oauth_pub = if config.include_google_oauth {
        "pub use oauth::*;"
    } else {
        ""
    };

    format!(
        r#"pub mod password;
{}
{}
{}

pub use password::*;
{}
{}
{}
"#,
        jwt_mod, session_mod, oauth_mod, jwt_pub, session_pub, oauth_pub
    )
}
