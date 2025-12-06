use crate::config::ProjectConfig;

pub fn generate(_config: &ProjectConfig) -> &'static str {
    r#"pub mod auth;
pub mod users;
pub mod health;

pub use auth::*;
pub use users::*;
pub use health::*;
"#
}
