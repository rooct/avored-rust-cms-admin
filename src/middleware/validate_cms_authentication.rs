// use std::sync::Arc;
// use axum::body::Body;
// use axum::extract::State;
// use axum::http::{header, Request, StatusCode};
// use axum::Json;
// use axum::middleware::Next;
// use axum::response::IntoResponse;
// use axum_extra::extract::CookieJar;
// use crate::avored_state::AvoRedState;
// use crate::middleware::require_jwt_authentication::ErrorResponse;
// use crate::models::setting_model::SettingModel;

// pub async fn validate_cms_authentication (
//     state: State<Arc<AvoRedState>>,
//     cookie_jar: CookieJar,
//     req: Request<Body>,
//     next: Next,
// ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

//     let token = cookie_jar
//         .get("token")
//         .map(|cookie| cookie.value().to_string())
//         .or_else(|| {
//             req.headers()
//                 .get(header::AUTHORIZATION)
//                 .and_then(|auth_header| auth_header.to_str().ok())
//                 .and_then(|auth_value| {
//                     if auth_value.starts_with("Bearer ") {
//                         match auth_value.strip_prefix("Bearer ") {
//                             Some(auth) => Some(auth.to_owned()),
//                             _ => None
//                         }
//                     } else {
//                         None
//                     }
//                 })
//         });

//     let token = token.ok_or_else(|| {
//         let json_error = ErrorResponse {
//             status: false,
//             message: "please provide token".to_string(),
//         };
//         (StatusCode::UNAUTHORIZED, Json(json_error))
//     })?;
//     let cms_token_setting_model = state
//         .setting_service
//         .find_by_identifier(&state.db, String::from("auth_cms_token"))
//         .await.unwrap_or_else(|_err|  {
//             SettingModel::default()
//         });

//     if cms_token_setting_model.value.is_empty() | cms_token_setting_model.value.ne(&token) {
//         let json_error = ErrorResponse {
//             status: false,
//             message: "please provide valid token".to_string(),
//         };
//         return Err((StatusCode::UNAUTHORIZED, Json(json_error)))
//     };

//     Ok(next.run(req).await)
// }
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::ErrorResponse;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use dashmap::DashMap;
use std::sync::{Arc, LazyLock};
use surrealdb::dbs::Session;

// 使用 DashMap 来存储每个会话的 CMS 令牌
static CMS_TOKEN_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(|| DashMap::new());

// 自定义错误类型
#[derive(Debug)]
pub enum AuthError {
    TokenMissing,
    TokenInvalid,
    DatabaseError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::TokenMissing => {
                (StatusCode::UNAUTHORIZED, "please provide token".to_string())
            }
            AuthError::TokenInvalid => (
                StatusCode::UNAUTHORIZED,
                "please provide valid token".to_string(),
            ),
            AuthError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        let json_error = ErrorResponse {
            status: false,
            message: message.to_string(),
        };
        (status, Json(json_error)).into_response()
    }
}

// 提取令牌
fn extract_token(cookie_jar: &CookieJar, req: &Request<Body>) -> Result<String, AuthError> {
    cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(|s| s.to_owned()))
        })
        .ok_or(AuthError::TokenMissing)
}

fn get_session_id(session: &Session) -> String {
    format!("{:?}-{:?}", session.ns, session.db)
}

// 从数据库或缓存中获取 CMS 令牌
async fn get_cms_token(state: &State<Arc<AvoRedState>>) -> Result<String, AuthError> {
    let session_id = get_session_id(&state.db.1);
    if let Some(cached_token) = CMS_TOKEN_CACHE.get(&session_id) {
        return Ok(cached_token.value().clone());
    }

    // 如果缓存中没有，从数据库中查询
    let cms_token_setting_model = state
        .setting_service
        .find_by_identifier(&state.db, "auth_cms_token".to_string())
        .await
        .map_err(|err| AuthError::DatabaseError(err.to_string()))?;

    let cms_token = cms_token_setting_model.value;
    if cms_token.is_empty() {
        return Err(AuthError::DatabaseError(
            "CMS token is not configured".to_string(),
        ));
    }

    // 将令牌存入缓存
    CMS_TOKEN_CACHE.insert(session_id.to_string(), cms_token.clone());
    Ok(cms_token)
}

// 验证令牌
fn validate_token(cms_token: &str, token: &str) -> Result<(), AuthError> {
    if cms_token != token {
        Err(AuthError::TokenInvalid)
    } else {
        Ok(())
    }
}

pub async fn validate_cms_authentication(
    state: State<Arc<AvoRedState>>,
    cookie_jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    // 提取令牌
    let token = extract_token(&cookie_jar, &req)?;
    // 获取 CMS 令牌
    let cms_token = get_cms_token(&state).await?;

    // 验证令牌
    validate_token(&cms_token, &token)?;

    // 继续处理请求
    Ok(next.run(req).await)
}
