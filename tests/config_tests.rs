use begin_rs_web::{AuthMethod, Database, ProjectConfig};

#[test]
fn test_database_as_str() {
    assert_eq!(Database::Postgres.as_str(), "postgres");
    assert_eq!(Database::MySql.as_str(), "mysql");
    assert_eq!(Database::Sqlite.as_str(), "sqlite");
    assert_eq!(Database::MongoDb.as_str(), "mongodb");
}

#[test]
fn test_database_connection_strings() {
    assert!(Database::Postgres
        .connection_string_template()
        .contains("postgresql://"));
    assert!(Database::MySql
        .connection_string_template()
        .contains("mysql://"));
    assert!(Database::Sqlite
        .connection_string_template()
        .contains("sqlite://"));
    assert!(Database::MongoDb
        .connection_string_template()
        .contains("mongodb://"));
}

#[test]
fn test_database_sqlx_features() {
    assert_eq!(Database::Postgres.sqlx_feature(), Some("postgres"));
    assert_eq!(Database::MySql.sqlx_feature(), Some("mysql"));
    assert_eq!(Database::Sqlite.sqlx_feature(), Some("sqlite"));
    assert_eq!(Database::MongoDb.sqlx_feature(), None);
}

#[test]
fn test_auth_method_supports_jwt() {
    assert!(AuthMethod::Jwt.supports_jwt());
    assert!(!AuthMethod::Session.supports_jwt());
    assert!(AuthMethod::Both.supports_jwt());
}

#[test]
fn test_auth_method_supports_session() {
    assert!(!AuthMethod::Jwt.supports_session());
    assert!(AuthMethod::Session.supports_session());
    assert!(AuthMethod::Both.supports_session());
}

#[test]
fn test_auth_method_as_str() {
    assert_eq!(AuthMethod::Jwt.as_str(), "jwt");
    assert_eq!(AuthMethod::Session.as_str(), "session");
    assert_eq!(AuthMethod::Both.as_str(), "both");
}

#[test]
fn test_project_config_creation() {
    let config = ProjectConfig {
        name: "test-app".to_string(),
        path: "./test-app".to_string(),
        database: Database::Postgres,
        auth_method: AuthMethod::Both,
        include_google_oauth: true,
        install_sqlx: false,
    };

    assert_eq!(config.name, "test-app");
    assert_eq!(config.path, "./test-app");
    assert_eq!(config.database, Database::Postgres);
    assert_eq!(config.auth_method, AuthMethod::Both);
    assert!(config.include_google_oauth);
}

#[test]
fn test_project_config_serialization() {
    let config = ProjectConfig {
        name: "test-app".to_string(),
        path: "./test-app".to_string(),
        database: Database::Sqlite,
        auth_method: AuthMethod::Jwt,
        include_google_oauth: false,
        install_sqlx: false,
    };

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ProjectConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.database, deserialized.database);
    assert_eq!(config.auth_method, deserialized.auth_method);
}
