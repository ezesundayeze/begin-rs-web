use begin_rs_web::{templates, AuthMethod, Database, ProjectConfig};

fn create_test_config(db: Database, auth: AuthMethod, oauth: bool) -> ProjectConfig {
    ProjectConfig {
        name: "test-app".to_string(),
        path: "./test-app".to_string(),
        database: db,
        auth_method: auth,
        include_google_oauth: oauth,
        install_sqlx: false,
    }
}

#[test]
fn test_cargo_toml_generation_postgres_jwt() {
    let config = create_test_config(Database::Postgres, AuthMethod::Jwt, false);
    let cargo_toml = templates::cargo_toml::generate(&config);

    assert!(cargo_toml.contains("name = \"test-app\""));
    assert!(cargo_toml.contains("axum"));
    assert!(cargo_toml.contains("sqlx"));
    assert!(cargo_toml.contains("postgres"));
    assert!(cargo_toml.contains("jsonwebtoken"));
    assert!(!cargo_toml.contains("redis"));
    assert!(!cargo_toml.contains("oauth2"));
}

#[test]
fn test_cargo_toml_generation_mongodb_session() {
    let config = create_test_config(Database::MongoDb, AuthMethod::Session, false);
    let cargo_toml = templates::cargo_toml::generate(&config);

    assert!(cargo_toml.contains("mongodb"));
    assert!(!cargo_toml.contains("sqlx"));
    assert!(cargo_toml.contains("redis"));
    assert!(!cargo_toml.contains("jsonwebtoken"));
}

#[test]
fn test_cargo_toml_generation_with_oauth() {
    let config = create_test_config(Database::Sqlite, AuthMethod::Both, true);
    let cargo_toml = templates::cargo_toml::generate(&config);

    assert!(cargo_toml.contains("oauth2"));
    assert!(cargo_toml.contains("reqwest"));
    assert!(cargo_toml.contains("jsonwebtoken"));
    assert!(cargo_toml.contains("redis"));
}

#[test]
fn test_error_rs_generation() {
    let error_content = templates::error_rs::generate();

    assert!(error_content.contains("pub enum AppError"));
    assert!(error_content.contains("DatabaseError"));
    assert!(error_content.contains("AuthenticationError"));
    assert!(error_content.contains("impl IntoResponse"));
    assert!(error_content.contains("impl std::error::Error"));
}

#[test]
fn test_config_rs_generation_jwt() {
    let config = create_test_config(Database::Postgres, AuthMethod::Jwt, false);
    let config_content = templates::config_rs::generate(&config);

    assert!(config_content.contains("pub jwt_secret: String"));
    assert!(config_content.contains("JWT_SECRET"));
    assert!(!config_content.contains("redis_url"));
}

#[test]
fn test_config_rs_generation_session() {
    let config = create_test_config(Database::Postgres, AuthMethod::Session, false);
    let config_content = templates::config_rs::generate(&config);

    assert!(config_content.contains("pub redis_url: String"));
    assert!(!config_content.contains("jwt_secret"));
}

#[test]
fn test_config_rs_generation_both() {
    let config = create_test_config(Database::Postgres, AuthMethod::Both, true);
    let config_content = templates::config_rs::generate(&config);

    assert!(config_content.contains("pub jwt_secret: String"));
    assert!(config_content.contains("pub redis_url: String"));
    assert!(config_content.contains("pub google_client_id"));
}

#[test]
fn test_env_generation() {
    let config = create_test_config(Database::Postgres, AuthMethod::Both, true);
    let env_content = templates::env::generate(&config);

    assert!(env_content.contains("DATABASE_URL=postgresql://"));
    assert!(env_content.contains("JWT_SECRET="));
    assert!(env_content.contains("REDIS_URL="));
    assert!(env_content.contains("GOOGLE_CLIENT_ID="));
}

#[test]
fn test_env_generation_minimal() {
    let config = create_test_config(Database::Sqlite, AuthMethod::Jwt, false);
    let env_content = templates::env::generate(&config);

    assert!(env_content.contains("DATABASE_URL=sqlite://"));
    assert!(env_content.contains("JWT_SECRET="));
    assert!(!env_content.contains("REDIS_URL="));
    assert!(!env_content.contains("GOOGLE_CLIENT_ID="));
}

#[test]
fn test_readme_generation() {
    let config = create_test_config(Database::Postgres, AuthMethod::Both, true);
    let readme = templates::readme::generate(&config);

    assert!(readme.contains("# test-app"));
    assert!(readme.contains("PostgreSQL"));
    assert!(readme.contains("JWT"));
    assert!(readme.contains("Session"));
    assert!(readme.contains("Google OAuth"));
}

#[test]
fn test_main_rs_generation_structure() {
    let config = create_test_config(Database::Postgres, AuthMethod::Both, true);
    let main_content = templates::main_rs::generate(&config);

    assert!(main_content.contains("mod config;"));
    assert!(main_content.contains("mod db;"));
    assert!(main_content.contains("mod auth;"));
    assert!(main_content.contains("mod routes;"));
    assert!(main_content.contains("mod middleware;"));
    assert!(main_content.contains("#[tokio::main]"));
    assert!(main_content.contains("async fn main()"));
}

#[test]
fn test_app_state_generation() {
    let config = create_test_config(Database::Postgres, AuthMethod::Both, true);
    let app_state = templates::app_state_rs::generate(&config);

    assert!(app_state.contains("pub struct AppState"));
    assert!(app_state.contains("pub pool: DbPool"));
    assert!(app_state.contains("pub config: Arc<Config>"));
    assert!(app_state.contains("pub session_store: Arc<SessionStore>"));
    assert!(app_state.contains("pub oauth_client: Arc<Option<GoogleOAuthClient>>"));
}

#[test]
fn test_app_state_generation_minimal() {
    let config = create_test_config(Database::Sqlite, AuthMethod::Jwt, false);
    let app_state = templates::app_state_rs::generate(&config);

    assert!(app_state.contains("pub struct AppState"));
    assert!(app_state.contains("pub pool: DbPool"));
    assert!(!app_state.contains("session_store"));
    assert!(!app_state.contains("oauth_client"));
}
