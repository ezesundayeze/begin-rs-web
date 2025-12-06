# Rust Web App Template Generator

A CLI tool for bootstrapping production-ready Rust web applications with Axum, authentication, user management, and database support.

## Features

- **Multiple Database Support**: PostgreSQL, MySQL, SQLite, and MongoDB
- **Flexible Authentication**: JWT tokens, session-based auth, or both (configurable)
- **Google OAuth Integration**: Optional OAuth support
- **Role-Based Access Control**: Built-in Admin, Moderator, and User roles
- **Security Best Practices**: Argon2 password hashing, secure session management
- **Production Ready**: Docker support, comprehensive error handling, migrations
- **Database Agnostic**: Trait-based design with SQLx for SQL databases, native MongoDB driver

## Installation

```bash
cargo install begin-rs-web
```

## Usage

### Interactive Mode (Recommended)

```bash
begin-rs-web
```

The CLI will guide you through selecting:
- Project name
- Database (PostgreSQL, MySQL, SQLite, or MongoDB)
- Authentication method (JWT, Sessions, or Both)
- Google OAuth (optional)

### Non-Interactive Mode

```bash
begin-rs-web my-app \
  --database postgres \
  --auth both \
  --google-oauth true \
  --non-interactive
```

### Available Options

```
Usage: begin-rs-web [OPTIONS] [NAME]

Arguments:
  [NAME]  Name of the project

Options:
  -d, --database <DATABASE>          Database to use (postgres, mysql, sqlite, mongodb)
  -a, --auth <AUTH>                  Authentication method (jwt, session, both)
  -n, --non-interactive              Skip interactive prompts
      --google-oauth <GOOGLE_OAUTH>  Include Google OAuth support [true/false]
      --path <PATH>                  Project path (defaults to ./<name>)
  -h, --help                         Print help
```

## Generated Project Structure

```
my-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Error handling
│   ├── app_state.rs         # Shared application state
│   ├── db/                  # Database connection & pool
│   ├── models/              # Data models (User, Session, etc.)
│   ├── auth/                # Authentication logic (JWT, Sessions, OAuth)
│   ├── routes/              # API route handlers
│   └── middleware/          # Custom middleware (auth, etc.)
├── migrations/              # Database migrations
├── Cargo.toml              # Dependencies
├── Dockerfile              # Docker container setup
├── docker-compose.yml      # Docker Compose configuration
├── .env.example            # Environment variables template
├── .gitignore             # Git ignore rules
└── README.md              # Project documentation
```

## What's Included

### Authentication

- **JWT**: Stateless token-based authentication
- **Sessions**: Redis-backed session management
- **OAuth**: Google OAuth 2.0 integration
- **Password Hashing**: Argon2 for secure password storage

### API Endpoints

#### Health Check
- `GET /health` - Server health status

#### Authentication
- `POST /auth/signup` - Create new user account
- `POST /auth/login` - Login with email/password
- `GET /auth/google` - Initiate Google OAuth flow (if enabled)
- `GET /auth/google/callback` - OAuth callback handler (if enabled)

#### User Management
- `GET /users/me` - Get current user (requires auth)
- `GET /users` - List all users (requires auth)
- `GET /users/:id` - Get user by ID (requires auth)
- `DELETE /users/:id` - Delete user (requires auth + admin role)

### User Roles

- **User**: Standard user with basic permissions
- **Moderator**: Can moderate content and users
- **Admin**: Full system access

### Database Support

#### SQL Databases (PostgreSQL, MySQL, SQLite)
- SQLx for compile-time verified queries
- Automatic migrations
- Connection pooling

#### MongoDB
- Official MongoDB driver
- Document-based storage
- Async operations

## Quick Start with Generated Project

```bash
# Navigate to generated project
cd my-app

# Setup environment
cp .env.example .env
# Edit .env with your configuration

# For SQL databases:
sqlx database create
sqlx migrate run

# Build and run
cargo build
cargo run
```

The server will start on `http://localhost:3000`

### Docker Setup

```bash
# Start all services (app + database)
docker-compose up

# Or build and run separately
docker build -t my-app .
docker run -p 3000:3000 --env-file .env my-app
```

## Environment Configuration

The generated `.env.example` includes:

```env
# Server
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Database
DATABASE_URL=postgresql://user:password@localhost/dbname

# JWT (if enabled)
JWT_SECRET=your-secret-key
JWT_EXPIRATION_HOURS=24

# Sessions (if enabled)
REDIS_URL=redis://127.0.0.1:6379
SESSION_EXPIRATION_HOURS=168

# Google OAuth (if enabled)
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback

# Logging
RUST_LOG=debug,tower_http=debug
```

## Example: Creating a New User

```bash
curl -X POST http://localhost:3000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "secure_password",
    "name": "John Doe"
  }'
```

## Example: Login

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "secure_password"
  }'
```

## Dependencies (Generated Project)

- **axum**: Web framework
- **tokio**: Async runtime
- **sqlx**: SQL database toolkit (for SQL databases)
- **mongodb**: MongoDB driver (for MongoDB)
- **jsonwebtoken**: JWT implementation (if JWT enabled)
- **redis**: Redis client (if sessions enabled)
- **oauth2**: OAuth 2.0 client (if OAuth enabled)
- **argon2**: Password hashing
- **serde**: Serialization framework
- **tower-http**: HTTP middleware
- **tracing**: Logging and diagnostics

## Features by Configuration

### PostgreSQL + JWT + Google OAuth
Full-featured setup with SQL database, stateless auth, and social login.

### SQLite + Sessions
Lightweight setup perfect for prototypes and small applications.

### MongoDB + Both Auth Methods
NoSQL database with maximum authentication flexibility.

## Notes

- The generated code compiles successfully with only minor warnings about unused helper methods
- All security best practices are followed (password hashing, secure cookies, CORS)
- Database migrations are included for SQL databases
- Docker configuration works out of the box
- Comprehensive error handling is built-in

## Development

```bash
# Run the CLI in development
cargo run -- my-app -d postgres -a jwt

# Build release binary
cargo build --release

# Test generated project
cd my-app && cargo check
```

## Contributing

The template generator is designed to be extensible. To add new features:

1. Update templates in `src/templates/`
2. Modify generation logic in `src/generator.rs`
3. Add configuration options in `src/config.rs`
4. Update CLI prompts in `src/prompts.rs`

## License

MIT
