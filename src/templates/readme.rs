use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let db_setup = match config.database {
        crate::config::Database::Postgres => {
            r#"## Database Setup

### PostgreSQL

1. Install PostgreSQL or use Docker:
```bash
docker run -d --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:16
```

2. Create database:
```bash
sqlx database create
```

3. Run migrations:
```bash
sqlx migrate run
```
"#
        }
        crate::config::Database::MySql => {
            r#"## Database Setup

### MySQL

1. Install MySQL or use Docker:
```bash
docker run -d --name mysql -e MYSQL_ROOT_PASSWORD=password -p 3306:3306 mysql:8
```

2. Create database:
```bash
sqlx database create
```

3. Run migrations:
```bash
sqlx migrate run
```
"#
        }
        crate::config::Database::Sqlite => {
            r#"## Database Setup

### SQLite

SQLite database will be created automatically when you run the application.

Run migrations:
```bash
sqlx migrate run
```
"#
        }
        crate::config::Database::MongoDb => {
            r#"## Database Setup

### MongoDB

1. Install MongoDB or use Docker:
```bash
docker run -d --name mongodb -p 27017:27017 mongo:7
```

2. MongoDB collections will be created automatically when you run the application.
"#
        }
    };

    let redis_setup = if config.auth_method.supports_session() {
        r#"
### Redis (for sessions)

Install Redis or use Docker:
```bash
docker run -d --name redis -p 6379:6379 redis:7-alpine
```
"#
    } else {
        ""
    };

    let auth_docs = match (config.auth_method.supports_jwt(), config.auth_method.supports_session()) {
        (true, true) => {
            r#"## Authentication

This project supports both JWT and session-based authentication:

### JWT Authentication

Include the JWT token in the Authorization header:
```
Authorization: Bearer <your-jwt-token>
```

### Session Authentication

The session token is stored in an HttpOnly cookie named `session_token`.
"#
        }
        (true, false) => {
            r#"## Authentication

This project uses JWT (JSON Web Tokens) for authentication.

Include the JWT token in the Authorization header:
```
Authorization: Bearer <your-jwt-token>
```
"#
        }
        (false, true) => {
            r#"## Authentication

This project uses session-based authentication with Redis.

The session token is stored in an HttpOnly cookie named `session_token`.
"#
        }
        _ => "",
    };

    let oauth_docs = if config.include_google_oauth {
        r#"
### Google OAuth

To enable Google OAuth, set the following environment variables:
- `GOOGLE_CLIENT_ID`: Your Google OAuth client ID
- `GOOGLE_CLIENT_SECRET`: Your Google OAuth client secret
- `GOOGLE_REDIRECT_URI`: The callback URL (e.g., http://localhost:3000/auth/google/callback)

Get these credentials from the [Google Cloud Console](https://console.cloud.google.com/).
"#
    } else {
        ""
    };

    let signup_endpoint = r#"#### POST /auth/signup
Create a new user account.

Request body:
```json
{
  "email": "user@example.com",
  "password": "secure_password",
  "name": "John Doe"
}
```
"#;

    let login_endpoint = r#"#### POST /auth/login
Log in with email and password.

Request body:
```json
{
  "email": "user@example.com",
  "password": "secure_password"
}
```
"#;

    let oauth_endpoints = if config.include_google_oauth {
        r#"
#### GET /auth/google
Initiate Google OAuth flow.

#### GET /auth/google/callback
Google OAuth callback endpoint.
"#
    } else {
        ""
    };

    format!(
        r#"# {}

A Rust web application built with Axum, featuring authentication, user management, and database support.

## Features

- RESTful API with Axum
- {} database integration
- {} authentication
- User roles and permissions (Admin, Moderator, User)
- {}Password hashing with Argon2
- Input validation
- Error handling
- Docker support
- CORS enabled

## Tech Stack

- **Framework**: Axum
- **Database**: {}{}
- **ORM**: SQLx
- **Authentication**: {}{}
- **Password Hashing**: Argon2

## Getting Started

### Prerequisites

- Rust 1.75 or higher
- {}{}

### Installation

1. Clone the repository:
```bash
cd {}
```

2. Copy the example environment file:
```bash
cp .env.example .env
```

3. Edit `.env` with your configuration

{}{}
4. Build and run:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Health Check

#### GET /health
Check if the server is running.

### Authentication

{}
{}{}
### Users

#### GET /users/me
Get current authenticated user's information.
Requires authentication.

#### GET /users
List all users.
Requires authentication.

#### GET /users/:id
Get a specific user by ID.
Requires authentication.

#### DELETE /users/:id
Delete a user (admin only).
Requires authentication and admin role.
{}{}
## User Roles

- **User**: Basic user with standard permissions
- **Moderator**: Can moderate content and users
- **Admin**: Full system access

## Development

### Running with Docker

```bash
docker-compose up
```

### Running Tests

```bash
cargo test
```

### Database Migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── error.rs          # Error handling
├── db/              # Database connection
├── models/          # Data models
├── auth/            # Authentication logic
├── routes/          # API route handlers
└── middleware/      # Custom middleware
```

## Environment Variables

See `.env.example` for all available configuration options.

## Security Notes

- Change `JWT_SECRET` in production
- Use strong database passwords
- Enable HTTPS in production
- Review CORS settings for production
- Regularly update dependencies

## License

MIT
"#,
        config.name,
        config.database.as_str(),
        config.auth_method.as_str(),
        if config.include_google_oauth { "Google OAuth support\n- " } else { "" },
        config.database.as_str(),
        if config.auth_method.supports_session() { "\n- **Cache**: Redis" } else { "" },
        config.auth_method.as_str(),
        if config.include_google_oauth { ", Google OAuth" } else { "" },
        config.database.as_str(),
        redis_setup,
        config.name,
        db_setup,
        redis_setup,
        signup_endpoint,
        login_endpoint,
        oauth_endpoints,
        auth_docs,
        oauth_docs
    )
}
