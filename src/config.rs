use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub path: String,
    pub database: Database,
    pub auth_method: AuthMethod,
    pub include_google_oauth: bool,
    pub install_sqlx: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Database {
    Postgres,
    MySql,
    Sqlite,
    MongoDb,
}

impl Database {
    pub fn as_str(&self) -> &'static str {
        match self {
            Database::Postgres => "postgres",
            Database::MySql => "mysql",
            Database::Sqlite => "sqlite",
            Database::MongoDb => "mongodb",
        }
    }

    pub fn connection_string_template(&self) -> &'static str {
        match self {
            Database::Postgres => "postgresql://user:password@localhost/dbname",
            Database::MySql => "mysql://user:password@localhost/dbname",
            Database::Sqlite => "sqlite://./database.db",
            Database::MongoDb => "mongodb://localhost:27017/dbname",
        }
    }

    pub fn sqlx_feature(&self) -> Option<&'static str> {
        match self {
            Database::Postgres => Some("postgres"),
            Database::MySql => Some("mysql"),
            Database::Sqlite => Some("sqlite"),
            Database::MongoDb => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AuthMethod {
    Jwt,
    Session,
    Both,
}

impl AuthMethod {
    pub fn supports_jwt(&self) -> bool {
        matches!(self, AuthMethod::Jwt | AuthMethod::Both)
    }

    pub fn supports_session(&self) -> bool {
        matches!(self, AuthMethod::Session | AuthMethod::Both)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AuthMethod::Jwt => "jwt",
            AuthMethod::Session => "session",
            AuthMethod::Both => "both",
        }
    }
}
