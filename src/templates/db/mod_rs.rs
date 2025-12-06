use crate::config::{Database, ProjectConfig};

pub fn generate(config: &ProjectConfig) -> &'static str {
    match config.database {
        Database::MongoDb => {
            r#"pub mod pool;

pub use pool::MongoPool;

pub type DbPool = MongoPool;
"#
        }
        _ => {
            r#"pub mod pool;

pub use pool::SqlPool;

pub type DbPool = SqlPool;
"#
        }
    }
}
