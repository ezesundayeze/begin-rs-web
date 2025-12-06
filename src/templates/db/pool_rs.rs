use crate::config::{Database, ProjectConfig};

pub fn generate(config: &ProjectConfig) -> String {
    match config.database {
        Database::MongoDb => generate_mongodb(),
        _ => generate_sqlx(config),
    }
}

fn generate_sqlx(config: &ProjectConfig) -> String {
    let pool_type = match config.database {
        Database::Postgres => "sqlx::PgPool",
        Database::MySql => "sqlx::MySqlPool",
        Database::Sqlite => "sqlx::SqlitePool",
        _ => unreachable!(),
    };

    format!(
        r#"use sqlx::{{Pool, migrate::MigrateDatabase}};
use anyhow::Result;

pub type SqlPool = {};

pub async fn create_pool(database_url: &str) -> Result<SqlPool> {{
    // Create database if it doesn't exist (mainly for SQLite)
    if !sqlx::any::Any::database_exists(database_url).await? {{
        tracing::info!("Creating database...");
        sqlx::any::Any::create_database(database_url).await?;
    }}

    let pool = Pool::connect(database_url).await?;

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("Database connection pool established");
    Ok(pool)
}}
"#,
        pool_type
    )
}

fn generate_mongodb() -> String {
    r#"use mongodb::{Client, Database};
use anyhow::Result;

#[derive(Clone)]
pub struct MongoPool {
    client: Client,
    database: Database,
}

impl MongoPool {
    pub async fn new(database_url: &str) -> Result<Self> {
        let client = Client::with_uri_str(database_url).await?;

        let db_name = database_url
            .split('/')
            .last()
            .unwrap_or("myapp")
            .split('?')
            .next()
            .unwrap_or("myapp");

        let database = client.database(db_name);

        tracing::info!("MongoDB connection established");

        Ok(Self { client, database })
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
"#
    .to_string()
}
