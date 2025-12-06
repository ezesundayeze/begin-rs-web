use crate::config::ProjectConfig;

pub fn generate(config: &ProjectConfig) -> String {
    let db_pool_creation = match config.database {
        crate::config::Database::MongoDb => {
            "let pool = db::pool::MongoPool::new(&app_config.database_url).await?;"
        }
        _ => {
            "let pool = db::pool::create_pool(&app_config.database_url).await?;"
        }
    };

    let session_setup = if config.auth_method.supports_session() {
        r#"
    let session_store = Arc::new(
        auth::SessionStore::new(&app_config.redis_url, app_config.session_expiration_hours).await?
    );"#
    } else {
        ""
    };

    let oauth_setup = if config.include_google_oauth {
        r#"
    let oauth_client = Arc::new(
        if let (Some(client_id), Some(client_secret), Some(redirect_uri)) = (
            app_config.google_client_id.clone(),
            app_config.google_client_secret.clone(),
            Some(auth::GoogleOAuthClient::new(client_id, client_secret, redirect_uri)?)
        } else {
            tracing::warn!("Google OAuth not configured");
            None
        }
    );"#
    } else {
        ""
    };

    let _state_fields = match (config.auth_method.supports_session(), config.include_google_oauth) {
        (true, true) => "pool, config, session_store, oauth_client",
        (true, false) => "pool, config, session_store, oauth_client: Arc::new(None)",
        (false, true) => "pool, config, session_store: Arc::new(SessionStore::new(\"\", 0).await?), oauth_client",
        (false, false) => "pool, config, session_store: Arc::new(SessionStore::new(\"\", 0).await?), oauth_client: Arc::new(None)",
    };

    let state_fields_actual = if config.auth_method.supports_session() && config.include_google_oauth {
        r#"pool,
        config,
        session_store,
        oauth_client,"#
    } else if config.auth_method.supports_session() {
        r#"pool,
        config,
        session_store,
        oauth_client: Arc::new(None),"#
    } else if config.include_google_oauth {
        r#"pool,
        config,
        oauth_client,"#
    } else {
        r#"pool,
        config,"#
    };

    let public_auth_routes = if config.include_google_oauth {
        r#"
        .route("/auth/signup", post(routes::signup))
        .route("/auth/login", post(routes::login))
        .route("/auth/google", get(routes::google_login))
        .route("/auth/google/callback", get(routes::google_callback))"#
    } else {
        r#"
        .route("/auth/signup", post(routes::signup))
        .route("/auth/login", post(routes::login))"#
    };

    format!(
        r#"mod config;
mod db;
mod error;
mod models;
mod auth;
mod routes;
mod middleware;
mod app_state;

use std::sync::Arc;
use axum::{{
    routing::{{get, post, delete}},
    Router,
}};
use tower_http::{{
    trace::TraceLayer,
    cors::CorsLayer,
}};

use config::Config;
use app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {{
    tracing_subscriber::fmt::init();

    let app_config = Config::from_env()?;
    let config = Arc::new(app_config.clone());

    tracing::info!("Connecting to database...");
    {}{}{}
    tracing::info!("Building application routes...");

    let state = AppState {{
        {}
    }};

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(routes::health_check)){}
        .with_state(state.clone());

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/users/me", get(routes::get_current_user))
        .route("/users", get(routes::list_users))
        .route("/users/:id", get(routes::get_user))
        .route("/users/:id", delete(routes::delete_user))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .with_state(state);

    // Combine public and protected routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = app_config.server_address();
    tracing::info!("Server starting on {{}}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}}
"#,
        db_pool_creation,
        session_setup,
        oauth_setup,
        state_fields_actual,
        public_auth_routes
    )
}
