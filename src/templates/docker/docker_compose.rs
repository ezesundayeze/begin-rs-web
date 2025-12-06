use crate::config::{Database, ProjectConfig};

pub fn generate(config: &ProjectConfig) -> String {
    let db_service = match config.database {
        Database::Postgres => {
            r#"  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: myapp
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
"#
        }
        Database::MySql => {
            r#"  mysql:
    image: mysql:8
    environment:
      MYSQL_ROOT_PASSWORD: password
      MYSQL_DATABASE: myapp
      MYSQL_USER: user
      MYSQL_PASSWORD: password
    ports:
      - "3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
"#
        }
        Database::Sqlite => {
            ""
        }
        Database::MongoDb => {
            r#"  mongodb:
    image: mongo:7
    environment:
      MONGO_INITDB_ROOT_USERNAME: user
      MONGO_INITDB_ROOT_PASSWORD: password
      MONGO_INITDB_DATABASE: myapp
    ports:
      - "27017:27017"
    volumes:
      - mongo_data:/data/db
"#
        }
    };

    let redis_service = if config.auth_method.supports_session() {
        r#"  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
"#
    } else {
        ""
    };

    let db_depends = match config.database {
        Database::Postgres => "      - postgres",
        Database::MySql => "      - mysql",
        Database::MongoDb => "      - mongodb",
        Database::Sqlite => "",
    };

    let redis_depends = if config.auth_method.supports_session() {
        "      - redis"
    } else {
        ""
    };

    let volumes = match (config.database, config.auth_method.supports_session()) {
        (Database::Postgres, true) => {
            r#"volumes:
  postgres_data:
  redis_data:
"#
        }
        (Database::Postgres, false) => {
            r#"volumes:
  postgres_data:
"#
        }
        (Database::MySql, true) => {
            r#"volumes:
  mysql_data:
  redis_data:
"#
        }
        (Database::MySql, false) => {
            r#"volumes:
  mysql_data:
"#
        }
        (Database::MongoDb, true) => {
            r#"volumes:
  mongo_data:
  redis_data:
"#
        }
        (Database::MongoDb, false) => {
            r#"volumes:
  mongo_data:
"#
        }
        (Database::Sqlite, true) => {
            r#"volumes:
  redis_data:
"#
        }
        (Database::Sqlite, false) => "",
    };

    format!(
        r#"version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=${{DATABASE_URL}}
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=3000
    depends_on:
{}
{}
    env_file:
      - .env
{}
{}
{}
"#,
        db_depends, redis_depends, db_service, redis_service, volumes
    )
}
