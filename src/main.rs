mod config;
mod generator;
mod prompts;
mod templates;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use config::{AuthMethod, Database, ProjectConfig};

#[derive(Parser, Debug)]
#[command(name = "create-rust-web")]
#[command(about = "Bootstrap a Rust web application with Axum, authentication, and database support", long_about = None)]
struct Cli {
    #[arg(help = "Name of the project")]
    name: Option<String>,

    #[arg(
        short,
        long,
        help = "Database to use (postgres, mysql, sqlite, mongodb)"
    )]
    database: Option<String>,

    #[arg(
        short,
        long,
        help = "Authentication method (jwt, session, both)"
    )]
    auth: Option<String>,

    #[arg(short, long, help = "Skip interactive prompts and use defaults/flags")]
    non_interactive: bool,

    #[arg(long, help = "Include Google OAuth support")]
    google_oauth: Option<bool>,

    #[arg(long, help = "Install sqlx-cli for the selected database (prompts if interactive)")]
    install_sqlx: Option<bool>,

    #[arg(long, help = "Project path (defaults to ./<name>)")]
    path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{}", "🦀 Rust Web App Template Generator".bright_blue().bold());
    println!();

    let config = if cli.non_interactive {
        create_config_from_flags(cli)?
    } else {
        create_config_interactive(cli)?
    };

    println!();
    println!("{}", "📦 Generating project...".bright_green());
    println!();

    generator::generate_project(&config)?;

    // Optionally install sqlx-cli for the chosen database
    if config.install_sqlx {
        if let Err(e) = install_sqlx_for_db(&config) {
            println!("Failed to install sqlx-cli: {}", e);
            println!("You can install it manually later. See the README for details.");
        }
    }

    println!();
    println!("{}", "✨ Project created successfully!".bright_green().bold());
    println!();
    println!("Next steps:");
    println!("  cd {}", config.name);
    println!("  cp .env.example .env");
    println!("  # Edit .env with your configuration");
    println!("  cargo build");
    println!("  sqlx database create");
    println!("  sqlx migrate run");
    println!("  cargo run");
    println!();
    println!("For more information, see the README.md in your new project.");

    Ok(())
}

fn install_sqlx_for_db(config: &ProjectConfig) -> Result<()> {
    use std::process::Command;
    use anyhow::Context;

    if let Some(feature) = config.database.sqlx_feature() {
        println!("Installing sqlx-cli for '{}'...", feature);

        // build feature list: native-tls + db feature
        let features = format!("native-tls,{}", feature);

        let status = Command::new("cargo")
            .arg("install")
            .arg("--locked")
            .arg("sqlx-cli")
            .arg("--no-default-features")
            .arg("--features")
            .arg(&features)
            .status()
            .context("Failed to spawn cargo install process")?;

        if status.success() {
            println!("sqlx-cli installed successfully. Make sure $HOME/.cargo/bin is on your PATH.");
            Ok(())
        } else {
            Err(anyhow::anyhow!("`cargo install sqlx-cli` returned non-zero exit status"))
        }
    } else {
        println!("Selected database does not require sqlx (skipping install).");
        Ok(())
    }
}

fn create_config_from_flags(cli: Cli) -> Result<ProjectConfig> {
    let name = cli.name.ok_or_else(|| anyhow::anyhow!("Project name is required"))?;

    let database = if let Some(db) = cli.database {
        match db.to_lowercase().as_str() {
            "postgres" | "postgresql" => Database::Postgres,
            "mysql" | "mariadb" => Database::MySql,
            "sqlite" => Database::Sqlite,
            "mongodb" | "mongo" => Database::MongoDb,
            _ => return Err(anyhow::anyhow!("Invalid database type")),
        }
    } else {
        Database::Postgres
    };

    let auth_method = if let Some(auth) = cli.auth {
        match auth.to_lowercase().as_str() {
            "jwt" => AuthMethod::Jwt,
            "session" => AuthMethod::Session,
            "both" => AuthMethod::Both,
            _ => return Err(anyhow::anyhow!("Invalid auth method")),
        }
    } else {
        AuthMethod::Both
    };

    let include_google_oauth = cli.google_oauth.unwrap_or(true);
    let install_sqlx = cli.install_sqlx.unwrap_or(false);
    let path = cli.path.unwrap_or_else(|| format!("./{}", name));

    Ok(ProjectConfig {
        name,
        path,
        database,
        auth_method,
        include_google_oauth,
        install_sqlx,
    })
}

fn create_config_interactive(cli: Cli) -> Result<ProjectConfig> {
    let name = if let Some(n) = cli.name {
        n
    } else {
        prompts::prompt_project_name()?
    };

    let database = if let Some(db) = cli.database {
        match db.to_lowercase().as_str() {
            "postgres" | "postgresql" => Database::Postgres,
            "mysql" | "mariadb" => Database::MySql,
            "sqlite" => Database::Sqlite,
            "mongodb" | "mongo" => Database::MongoDb,
            _ => return Err(anyhow::anyhow!("Invalid database type")),
        }
    } else {
        prompts::prompt_database()?
    };

    let auth_method = if let Some(auth) = cli.auth {
        match auth.to_lowercase().as_str() {
            "jwt" => AuthMethod::Jwt,
            "session" => AuthMethod::Session,
            "both" => AuthMethod::Both,
            _ => return Err(anyhow::anyhow!("Invalid auth method")),
        }
    } else {
        prompts::prompt_auth_method()?
    };

    let include_google_oauth = if let Some(oauth) = cli.google_oauth {
        oauth
    } else {
        prompts::prompt_google_oauth()?
    };

    let install_sqlx = if let Some(install) = cli.install_sqlx {
        install
    } else {
        // only prompt if the selected database supports sqlx
        if database.sqlx_feature().is_some() {
            prompts::prompt_install_sqlx()?
        } else {
            false
        }
    };

    let path = cli.path.unwrap_or_else(|| format!("./{}", name));

    Ok(ProjectConfig {
        name,
        path,
        database,
        auth_method,
        include_google_oauth,
        install_sqlx,
    })
}
