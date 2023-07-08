use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::handlers::home_handler::home_handler;
use crate::handlers::roles_handler::roles_handler;
use crate::middleware::require_authentication::require_authentication;
use crate::repositories::admin_user_repository::{AdminUser, AdminUserRepository};

use crate::handlers::admin_users_handler::admin_users_handler;
use crate::handlers::create_admin_user_handler::create_admin_user_handler;
use crate::handlers::delete_admin_user_handler::delete_admin_user_handler;
use crate::handlers::get_admin_user_handler::get_admin_user_handler;
use crate::handlers::login_admin_user_handler::login_admin_user_handler;
use crate::handlers::put_admin_user_handler::put_admin_user_handler;
use crate::repositories::role_repository::RoleRepository;
use sea_orm::Database;

use crate::config::Config;

pub struct AppState {
    pub admin_user_repository: AdminUserRepository,
    pub role_repository: RoleRepository,
    // pub connection: sea_orm::DatabaseConnection,
    pub config: Config,
    pub current_user: Option<AdminUser>,
}

pub async fn app_routes() -> Router {
    let config: Config = Config::new();

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(config.backend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);

    let db = establish_connection().await;
    // let connection = &mut db.get().unwrap();

    /************** REPOSITORIES  **************/
    let admin_user_repository = AdminUserRepository::new();
    let role_repository = RoleRepository::new();

    let config: Config = Config::new();

    /************** APPLICATION STATES  **************/
    let app_state = Arc::new(AppState {
        admin_user_repository,
        role_repository,
        // connection: db,
        config,
        current_user: None,
    });

    Router::new()
        .route("/", get(home_handler))
        .route("/api/roles", get(roles_handler))
        .route("/api/admin-users", get(admin_users_handler))
        .route(
            "/api/admin-users/:admin_user_id",
            get(get_admin_user_handler),
        )
        .route(
            "/api/admin-users/:admin_user_id",
            put(put_admin_user_handler),
        )
        .route(
            "/api/admin-users/:admin_user_id",
            delete(delete_admin_user_handler),
        )
        .route("/api/admin-users", post(create_admin_user_handler))
        // ABOVE ROUTES ARE AUTH MIDDLEWARE
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/api/auth/login", post(login_admin_user_handler))
        .with_state(app_state)
        .layer(cors)
}

pub async fn establish_connection() -> sea_orm::DatabaseConnection {
    let config: Config = Config::new();
    let database_url: String = config.database_url;

    // let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    // r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool.")

    // let database_url = std::env::var("DATABASE_URL").unwrap();
    let db: sea_orm::DatabaseConnection = Database::connect(&database_url).await.unwrap();
    // .expect("Failed to setup the database")

    db
}
