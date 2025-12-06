use begin_rs_web::{generator, AuthMethod, Database, ProjectConfig};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn create_test_config_in_temp(
    temp_dir: &TempDir,
    name: &str,
    db: Database,
    auth: AuthMethod,
    oauth: bool,
) -> ProjectConfig {
    let path = temp_dir.path().join(name);
    ProjectConfig {
        name: name.to_string(),
        path: path.to_string_lossy().to_string(),
        database: db,
        auth_method: auth,
        include_google_oauth: oauth,
        install_sqlx: false,
    }
}

#[test]
fn test_generate_project_creates_directory() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let project_path = Path::new(&config.path);
    assert!(project_path.exists());
    assert!(project_path.is_dir());
}

#[test]
fn test_generate_project_creates_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let cargo_toml = Path::new(&config.path).join("Cargo.toml");
    assert!(cargo_toml.exists());

    let content = fs::read_to_string(cargo_toml).unwrap();
    assert!(content.contains("name = \"test-app\""));
    assert!(content.contains("axum"));
}

#[test]
fn test_generate_project_creates_src_directory() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Sqlite, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let src_dir = Path::new(&config.path).join("src");
    assert!(src_dir.exists());
    assert!(src_dir.is_dir());

    // Check main files
    assert!(src_dir.join("main.rs").exists());
    assert!(src_dir.join("config.rs").exists());
    assert!(src_dir.join("error.rs").exists());
    assert!(src_dir.join("app_state.rs").exists());
}

#[test]
fn test_generate_project_creates_modules() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Both, true);

    generator::generate_project(&config).unwrap();

    let src_dir = Path::new(&config.path).join("src");

    // Check module directories
    assert!(src_dir.join("db").exists());
    assert!(src_dir.join("auth").exists());
    assert!(src_dir.join("routes").exists());
    assert!(src_dir.join("models").exists());
    assert!(src_dir.join("middleware").exists());
}

#[test]
fn test_generate_project_creates_migrations() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Session, false);

    generator::generate_project(&config).unwrap();

    let migrations_dir = Path::new(&config.path).join("migrations");
    assert!(migrations_dir.exists());
    assert!(migrations_dir.join("00001_create_users.sql").exists());
    assert!(migrations_dir.join("00002_create_sessions.sql").exists());
}

#[test]
fn test_generate_project_creates_env_example() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::MySql, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let env_file = Path::new(&config.path).join(".env.example");
    assert!(env_file.exists());

    let content = fs::read_to_string(env_file).unwrap();
    assert!(content.contains("DATABASE_URL="));
    assert!(content.contains("SERVER_HOST="));
}

#[test]
fn test_generate_project_creates_docker_files() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let project_path = Path::new(&config.path);
    assert!(project_path.join("Dockerfile").exists());
    assert!(project_path.join("docker-compose.yml").exists());
}

#[test]
fn test_generate_project_creates_readme() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Sqlite, AuthMethod::Both, false);

    generator::generate_project(&config).unwrap();

    let readme = Path::new(&config.path).join("README.md");
    assert!(readme.exists());

    let content = fs::read_to_string(readme).unwrap();
    assert!(content.contains("# test-app"));
}

#[test]
fn test_generate_project_creates_gitignore() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let gitignore = Path::new(&config.path).join(".gitignore");
    assert!(gitignore.exists());

    let content = fs::read_to_string(gitignore).unwrap();
    assert!(content.contains("/target"));
    assert!(content.contains(".env"));
}

#[test]
fn test_generate_project_with_oauth_creates_oauth_module() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::Postgres, AuthMethod::Jwt, true);

    generator::generate_project(&config).unwrap();

    let oauth_file = Path::new(&config.path).join("src/auth/oauth.rs");
    assert!(oauth_file.exists());

    let content = fs::read_to_string(oauth_file).unwrap();
    assert!(content.contains("GoogleOAuthClient"));
}

#[test]
fn test_generate_project_mongodb_no_migrations() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config_in_temp(&temp_dir, "test-app", Database::MongoDb, AuthMethod::Jwt, false);

    generator::generate_project(&config).unwrap();

    let migrations_dir = Path::new(&config.path).join("migrations");

    // Migrations directory exists but should be empty or have no SQL files
    if migrations_dir.exists() {
        let entries: Vec<_> = fs::read_dir(migrations_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "sql").unwrap_or(false))
            .collect();
        assert!(entries.is_empty(), "MongoDB should not have SQL migration files");
    }
}

#[test]
fn test_generate_multiple_projects_no_conflict() {
    let temp_dir = TempDir::new().unwrap();

    let config1 = create_test_config_in_temp(&temp_dir, "app1", Database::Postgres, AuthMethod::Jwt, false);
    let config2 = create_test_config_in_temp(&temp_dir, "app2", Database::MySql, AuthMethod::Session, true);

    generator::generate_project(&config1).unwrap();
    generator::generate_project(&config2).unwrap();

    assert!(Path::new(&config1.path).exists());
    assert!(Path::new(&config2.path).exists());

    // Verify they're independent
    let cargo1 = fs::read_to_string(Path::new(&config1.path).join("Cargo.toml")).unwrap();
    let cargo2 = fs::read_to_string(Path::new(&config2.path).join("Cargo.toml")).unwrap();

    assert!(cargo1.contains("app1"));
    assert!(cargo2.contains("app2"));
    assert!(cargo1.contains("postgres"));
    assert!(cargo2.contains("mysql"));
}
