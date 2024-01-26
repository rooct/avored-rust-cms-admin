use std::sync::Arc;

use axum::{routing::get, Router, middleware};
use axum::routing::{post, put};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::header::HeaderValue;
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use tower_http::cors::CorsLayer;
use crate::api::rest_api::handlers::{
    page::page_table_api_handler::page_table_api_handler,
    admin_user::admin_user_login_api_handler::admin_user_login_api_handler,
    component_all_api_handler::component_all_api_handler,
    health_check_api_handler::health_check_api_handler,
    page::store_page_api_handler::store_page_api_handler,
    page::update_page_api_handler::update_page_api_handler,
    page::fetch_page_api_handler::fetch_page_api_handler,
    admin_user::admin_user_table_api_handler::admin_user_table_api_handler,
    admin_user::store_admin_user_api_handler::store_admin_user_api_handler,
    admin_user::update_admin_user_api_handler::update_admin_user_api_handler,
    admin_user::fetch_admin_user_api_handler::fetch_admin_user_api_handler,
    role::role_table_api_handler::role_table_api_handler,
    role::fetch_role_api_handler::fetch_role_api_handler,
    role::store_role_api_handler::store_role_api_handler,
    role::update_role_api_handler::update_role_api_handler,
    role::role_option_api_handler::role_option_api_handler,
    asset::asset_table_api_handler::asset_table_api_handler,
    asset::store_asset_api_handler::store_asset_api_handler,
    component::component_table_api_handler::component_table_api_handler,
    component::store_component_api_handler::store_component_api_handler,
    component::fetch_component_api_handler::fetch_component_api_handler,
    component::update_component_api_handler::update_component_api_handler,
    setup::post_setup_avored_handler::post_setup_avored_handler
};

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {

    let backend_url = &state.config.front_end_app_url;

    println!("BACKEND URL {backend_url}");
    let cors_layer = CorsLayer::new()
        .allow_origin(backend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);

    Router::new()
        .route("/api/component", get(component_table_api_handler))
        .route("/api/component", post(store_component_api_handler))
        .route("/api/component/:component_id", get(fetch_component_api_handler))
        .route("/api/component/:component_id", put(update_component_api_handler))
        .route("/api/asset", get(asset_table_api_handler))
        .route("/api/asset", post(store_asset_api_handler))
        .route("/api/role-options", get(role_option_api_handler))
        .route("/api/role", get(role_table_api_handler))
        .route("/api/role", post(store_role_api_handler))
        .route("/api/role/:role_id", get(fetch_role_api_handler))
        .route("/api/role/:role_id", put(update_role_api_handler))
        .route("/api/admin-user", get(admin_user_table_api_handler))
        .route("/api/admin-user", post(store_admin_user_api_handler))
        .route("/api/admin-user/:admin_user_id", put(update_admin_user_api_handler))
        .route("/api/admin-user/:admin_user_id", get(fetch_admin_user_api_handler))
        .route("/api/page", get(page_table_api_handler))
        .route("/api/page", post(store_page_api_handler))
        .route("/api/page/:page_id", put(update_page_api_handler))
        .route("/api/page/:page_id", get(fetch_page_api_handler))
        .route("/api/component-all", get(component_all_api_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .route("/api/health-check", get(health_check_api_handler))
        .route("/api/setup", post(post_setup_avored_handler))
        .route("/api/login", post(admin_user_login_api_handler))
        .with_state(state)
        .layer(cors_layer)
}
