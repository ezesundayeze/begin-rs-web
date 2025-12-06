use crate::config::ProjectConfig;

pub fn generate(_config: &ProjectConfig) -> &'static str {
    r#"pub mod auth;

pub use auth::*;
"#
}
