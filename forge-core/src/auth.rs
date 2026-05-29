//! 认证中间件 — API Key + JWT 双模认证
//!
//! - API Key: 静态密钥，用于 Agent/服务调用，通过 `X-API-Key` 请求头传递
//! - JWT: 动态令牌，用于 Web Studio，通过 `Authorization: Bearer <token>` 传递
//! - /health 端点免认证

use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::ForgeError;

/// JWT 配置
#[derive(Clone)]
pub struct AuthConfig {
    /// API Key（静态密钥，用于 Agent 调用）
    pub api_key: String,
    /// JWT 签名密钥
    pub jwt_secret: String,
    /// JWT 过期时间（小时）
    pub jwt_expire_hours: i64,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("FORGE_API_KEY")
                .unwrap_or_else(|_| "forge-dev-api-key-change-in-production".to_string()),
            jwt_secret: std::env::var("FORGE_JWT_SECRET")
                .unwrap_or_else(|_| "forge-dev-jwt-secret-change-in-production".to_string()),
            jwt_expire_hours: std::env::var("FORGE_JWT_EXPIRE_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
        }
    }
}

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// 主体（用户/Agent ID）
    pub sub: String,
    /// 签发时间
    pub iat: i64,
    /// 过期时间
    pub exp: i64,
}

/// 创建 JWT Token
pub fn create_jwt(config: &AuthConfig, subject: &str) -> Result<String, ForgeError> {
    let now = Utc::now();
    let claims = JwtClaims {
        sub: subject.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::hours(config.jwt_expire_hours)).timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ForgeError::Internal(format!("JWT encode failed: {}", e)))
}

/// 验证 JWT Token
pub fn verify_jwt(config: &AuthConfig, token: &str) -> Result<JwtClaims, ForgeError> {
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ForgeError::Unauthorized)
}

/// 认证主体
#[derive(Debug, Clone)]
pub struct AuthSubject {
    pub id: String,
    pub auth_type: AuthType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthType {
    ApiKey,
    Jwt,
}

/// 认证失败响应
pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "error": {
                    "code": "UNAUTHORIZED",
                    "message": "Valid API key or JWT token required"
                }
            })),
        )
            .into_response()
    }
}

/// 从请求头中提取认证信息（纯函数，不用 trait）
pub fn extract_auth(parts: &Parts, config: &AuthConfig) -> Result<AuthSubject, AuthError> {
    // 1. 尝试 API Key
    if let Some(api_key) = parts.headers.get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            if key_str == config.api_key {
                return Ok(AuthSubject {
                    id: "api-agent".to_string(),
                    auth_type: AuthType::ApiKey,
                });
            }
        }
    }

    // 2. 尝试 JWT Bearer Token
    if let Some(auth_header) = parts.headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = verify_jwt(config, token) {
                    return Ok(AuthSubject {
                        id: claims.sub,
                        auth_type: AuthType::Jwt,
                    });
                }
            }
        }
    }

    // 3. 尝试 Query 参数 token（用于 WebSocket）
    let query = parts.uri.query().unwrap_or("");
    for pair in query.split('&') {
        if let Some(token) = pair.strip_prefix("token=") {
            if let Ok(claims) = verify_jwt(config, token) {
                return Ok(AuthSubject {
                    id: claims.sub,
                    auth_type: AuthType::Jwt,
                });
            }
        }
    }

    Err(AuthError)
}

/// Token 请求
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    /// API Key 用于换取 JWT
    pub api_key: String,
    /// 主体标识（可选，默认 "web-studio"）
    pub subject: Option<String>,
}

/// Token 响应
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_in: i64,
    pub token_type: String,
}
