use crate::config::{Database, ProjectConfig};

pub fn generate(config: &ProjectConfig) -> String {
    match config.database {
        Database::MongoDb => generate_mongodb(),
        _ => generate_sqlx(config),
    }
}

fn generate_sqlx(_config: &ProjectConfig) -> String {
    r#"use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            "moderator" => Some(UserRole::Moderator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub name: String,
    pub role: String,
    pub google_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn get_role(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap_or(UserRole::User)
    }

    pub fn is_admin(&self) -> bool {
        self.get_role() == UserRole::Admin
    }

    pub fn is_moderator(&self) -> bool {
        matches!(self.get_role(), UserRole::Admin | UserRole::Moderator)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            created_at: user.created_at,
        }
    }
}
"#
    .to_string()
}

fn generate_mongodb() -> String {
    r#"use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            "moderator" => Some(UserRole::Moderator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub name: String,
    pub role: String,
    pub google_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn get_role(&self) -> UserRole {
        UserRole::from_str(&self.role).unwrap_or(UserRole::User)
    }

    pub fn is_admin(&self) -> bool {
        self.get_role() == UserRole::Admin
    }

    pub fn is_moderator(&self) -> bool {
        matches!(self.get_role(), UserRole::Admin | UserRole::Moderator)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.map(|id| id.to_hex()).unwrap_or_default(),
            email: user.email,
            name: user.name,
            role: user.role,
            created_at: user.created_at,
        }
    }
}
"#
    .to_string()
}
