use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::config::ProjectConfig;
use crate::templates;

pub fn generate_project(config: &ProjectConfig) -> Result<()> {
    let project_path = Path::new(&config.path);

    fs::create_dir_all(project_path)
        .context("Failed to create project directory")?;

    generate_cargo_toml(config, project_path)?;
    generate_src_directory(config, project_path)?;
    generate_migrations(config, project_path)?;
    generate_env_files(config, project_path)?;
    generate_readme(config, project_path)?;
    generate_gitignore(project_path)?;
    generate_docker_files(config, project_path)?;

    Ok(())
}

fn generate_cargo_toml(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    let content = templates::cargo_toml::generate(config);
    fs::write(project_path.join("Cargo.toml"), content)
        .context("Failed to write Cargo.toml")?;
    Ok(())
}

fn generate_src_directory(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path)?;

    fs::write(src_path.join("main.rs"), templates::main_rs::generate(config))?;
    fs::write(src_path.join("config.rs"), templates::config_rs::generate(config))?;
    fs::write(src_path.join("error.rs"), templates::error_rs::generate())?;
    fs::write(src_path.join("app_state.rs"), templates::app_state_rs::generate(config))?;

    generate_db_module(config, &src_path)?;
    generate_auth_module(config, &src_path)?;
    generate_routes_module(config, &src_path)?;
    generate_models_module(config, &src_path)?;
    generate_middleware_module(config, &src_path)?;

    Ok(())
}

fn generate_db_module(config: &ProjectConfig, src_path: &Path) -> Result<()> {
    let db_path = src_path.join("db");
    fs::create_dir_all(&db_path)?;

    fs::write(db_path.join("mod.rs"), templates::db::mod_rs::generate(config))?;
    fs::write(db_path.join("pool.rs"), templates::db::pool_rs::generate(config))?;

    Ok(())
}

fn generate_auth_module(config: &ProjectConfig, src_path: &Path) -> Result<()> {
    let auth_path = src_path.join("auth");
    fs::create_dir_all(&auth_path)?;

    fs::write(auth_path.join("mod.rs"), templates::auth::mod_rs::generate(config))?;

    if config.auth_method.supports_jwt() {
        fs::write(auth_path.join("jwt.rs"), templates::auth::jwt_rs::generate())?;
    }

    if config.auth_method.supports_session() {
        fs::write(auth_path.join("session.rs"), templates::auth::session_rs::generate(config))?;
    }

    fs::write(auth_path.join("password.rs"), templates::auth::password_rs::generate())?;

    if config.include_google_oauth {
        fs::write(auth_path.join("oauth.rs"), templates::auth::oauth_rs::generate())?;
    }

    Ok(())
}

fn generate_routes_module(config: &ProjectConfig, src_path: &Path) -> Result<()> {
    let routes_path = src_path.join("routes");
    fs::create_dir_all(&routes_path)?;

    fs::write(routes_path.join("mod.rs"), templates::routes::mod_rs::generate(config))?;
    fs::write(routes_path.join("auth.rs"), templates::routes::auth_rs::generate(config))?;
    fs::write(routes_path.join("users.rs"), templates::routes::users_rs::generate(config))?;
    fs::write(routes_path.join("health.rs"), templates::routes::health_rs::generate())?;

    Ok(())
}

fn generate_models_module(config: &ProjectConfig, src_path: &Path) -> Result<()> {
    let models_path = src_path.join("models");
    fs::create_dir_all(&models_path)?;

    fs::write(models_path.join("mod.rs"), templates::models::mod_rs::generate())?;
    fs::write(models_path.join("user.rs"), templates::models::user_rs::generate(config))?;

    if config.auth_method.supports_session() {
        fs::write(models_path.join("session.rs"), templates::models::session_rs::generate(config))?;
    }

    Ok(())
}

fn generate_middleware_module(config: &ProjectConfig, src_path: &Path) -> Result<()> {
    let middleware_path = src_path.join("middleware");
    fs::create_dir_all(&middleware_path)?;

    fs::write(middleware_path.join("mod.rs"), templates::middleware::mod_rs::generate(config))?;
    fs::write(middleware_path.join("auth.rs"), templates::middleware::auth_rs::generate(config))?;

    Ok(())
}

fn generate_migrations(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    // MongoDB doesn't use SQL migrations
    if matches!(config.database, crate::config::Database::MongoDb) {
        return Ok(());
    }

    let migrations_path = project_path.join("migrations");
    fs::create_dir_all(&migrations_path)?;

    fs::write(
        migrations_path.join("00001_create_users.sql"),
        templates::migrations::create_users_sql::generate(config),
    )?;

    if config.auth_method.supports_session() {
        fs::write(
            migrations_path.join("00002_create_sessions.sql"),
            templates::migrations::create_sessions_sql::generate(config),
        )?;
    }

    Ok(())
}

fn generate_env_files(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    fs::write(
        project_path.join(".env.example"),
        templates::env::generate(config),
    )?;

    Ok(())
}

fn generate_readme(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    fs::write(
        project_path.join("README.md"),
        templates::readme::generate(config),
    )?;

    Ok(())
}

fn generate_gitignore(project_path: &Path) -> Result<()> {
    let gitignore_content = r#"# Rust
/target
**/*.rs.bk
*.pdb
Cargo.lock

# Environment
.env
.env.local

# Database
*.db
*.db-shm
*.db-wal

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
"#;

    fs::write(project_path.join(".gitignore"), gitignore_content)?;

    Ok(())
}

fn generate_docker_files(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    fs::write(
        project_path.join("Dockerfile"),
        templates::docker::dockerfile::generate(config),
    )?;

    fs::write(
        project_path.join("docker-compose.yml"),
        templates::docker::docker_compose::generate(config),
    )?;

    Ok(())
}
