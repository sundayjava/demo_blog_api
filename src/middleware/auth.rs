use crate::models::user::Role;
use crate::services::jwt::Claims;
use crate::services::jwt::JwtService;
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::sync::Arc;
use uuid::Uuid;

pub struct JwtAuth {
    jwt_service: Arc<JwtService>,
}

impl JwtAuth {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            jwt_service: self.jwt_service.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    jwt_service: Arc<JwtService>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_service = self.jwt_service.clone();

        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = token {
            match jwt_service.verify_token(token) {
                Ok(claims) => {
                    // Store claims in request extensions
                    req.extensions_mut().insert(claims);
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(e) => {
                    tracing::warn!("Invalid JWT token: {}", e);
                    Box::pin(async {
                        Err(actix_web::error::ErrorUnauthorized(
                            "Invalid or expired token",
                        ))
                    })
                }
            }
        } else {
            Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Missing authorization token",
                ))
            })
        }
    }
}

// Extractor for authenticated user
use actix_web::{FromRequest, HttpRequest, dev::Payload};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: Role,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            let role = Role::from_str(&claims.role).unwrap_or(Role::User);

            let user_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(_) => {
                    return ready(Err(actix_web::error::ErrorUnauthorized("Invalid user ID")));
                }
            };

            ready(Ok(AuthenticatedUser {
                user_id,
                email: claims.email.clone(),
                role,
            }))
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
        }
    }
}

impl AuthenticatedUser {
    pub fn _has_permission(&self, required_role: &Role) -> bool {
        self.role._has_permission(required_role)
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_moderator_or_above(&self) -> bool {
        matches!(self.role, Role::Admin | Role::Moderator)
    }
}
