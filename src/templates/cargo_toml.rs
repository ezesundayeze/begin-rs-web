use crate::config::{Database, ProjectConfig};

pub fn generate(config: &ProjectConfig) -> String {
    let sqlx_features = match config.database {
        Database::MongoDb => String::new(),
        db => {
            let feature = db.sqlx_feature().unwrap();
            format!(
                r#"sqlx = {{ version = "0.7", features = ["runtime-tokio-rustls", "{}", "uuid", "chrono", "migrate"] }}"#,
                feature
            )
        }
    };

    let mongodb_deps = if config.database == Database::MongoDb {
        r#"mongodb = "2.8"
bson = { version = "2.9", features = ["chrono-0_4", "uuid-1"] }"#
    } else {
        ""
    };

    let jwt_deps = if config.auth_method.supports_jwt() {
        r#"jsonwebtoken = "9.2"
"#
    } else {
        ""
    };

    let session_deps = if config.auth_method.supports_session() {
        r#"redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
"#
    } else {
        ""
    };

    let oauth_deps = if config.include_google_oauth {
        r#"oauth2 = "4.4"
reqwest = { version = "0.11", features = ["json"] }
"#
    } else {
        ""
    };

    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = {{ version = "0.7", features = ["macros"] }}
tokio = {{ version = "1", features = ["full"] }}
tower = {{ version = "0.4", features = ["util"] }}
tower-http = {{ version = "0.5", features = ["fs", "cors", "trace"] }}

# Database
{}
{}

# Serialization
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

# Authentication & Security
{}{}{}argon2 = "0.5"
uuid = {{ version = "1.6", features = ["v4", "serde"] }}

# Configuration
dotenvy = "0.15"

# Logging & Tracing
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Date/Time
chrono = {{ version = "0.4", features = ["serde"] }}

# Validation
validator = {{ version = "0.18", features = ["derive"] }}

[dev-dependencies]
# Testing
tower = {{ version = "0.4", features = ["util"] }}
hyper = {{ version = "1.1", features = ["full"] }}
http-body-util = "0.1"
"#,
        config.name,
        sqlx_features,
        mongodb_deps,
        jwt_deps,
        session_deps,
        oauth_deps,
    )
}
