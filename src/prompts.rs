use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::config::{AuthMethod, Database};

pub fn prompt_project_name() -> Result<String> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("my-web-app".to_string())
        .interact_text()?;

    Ok(name)
}

pub fn prompt_database() -> Result<Database> {
    let databases = vec![
        "PostgreSQL (Recommended for production)",
        "MySQL/MariaDB",
        "SQLite (Great for development/small apps)",
        "MongoDB (NoSQL)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a database")
        .default(0)
        .items(&databases)
        .interact()?;

    let database = match selection {
        0 => Database::Postgres,
        1 => Database::MySql,
        2 => Database::Sqlite,
        3 => Database::MongoDb,
        _ => unreachable!(),
    };

    Ok(database)
}

pub fn prompt_auth_method() -> Result<AuthMethod> {
    let methods = vec![
        "Both JWT and Sessions (Maximum flexibility)",
        "JWT only (Stateless, good for APIs)",
        "Sessions only (Traditional, easier to revoke)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Authentication method")
        .default(0)
        .items(&methods)
        .interact()?;

    let auth_method = match selection {
        0 => AuthMethod::Both,
        1 => AuthMethod::Jwt,
        2 => AuthMethod::Session,
        _ => unreachable!(),
    };

    Ok(auth_method)
}

pub fn prompt_google_oauth() -> Result<bool> {
    let include = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Include Google OAuth support?")
        .default(true)
        .interact()?;

    Ok(include)
}

pub fn prompt_install_sqlx() -> Result<bool> {
    let install = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Install `sqlx-cli` for the selected database?")
        .default(true)
        .interact()?;

    Ok(install)
}
