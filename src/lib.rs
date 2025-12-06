// Re-export modules for testing
pub mod config;
pub mod generator;
pub mod prompts;
pub mod templates;

// Make internal modules available for integration tests
pub use config::{AuthMethod, Database, ProjectConfig};
