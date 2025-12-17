//! Actix-web middleware implementation

use super::store::RateLimiter;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use tracing::{debug, warn};

// == MIDDLEWARE FACTORY // ==

/// Rate limiting middleware for Actix-web
#[derive(Clone)]
pub struct RateLimitMiddleware {
    limiter: RateLimiter,
}

impl RateLimitMiddleware {
    /// Create new middleware with the given rate limiter
    pub fn new(limiter: RateLimiter) -> Self {
        Self { limiter }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service: Rc::new(service),
            limiter: self.limiter.clone(),
        }))
    }
}

// == MIDDLEWARE SERVICE // ==

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
    limiter: RateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let limiter = self.limiter.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Extract client identifier (prefer user ID from session, fallback to IP)
            let client_key = extract_client_key(&req);
            let path = req.path().to_string();

            // Check rate limit
            if let Err(e) = limiter.check(&path, &client_key) {
                warn!(
                    client = %client_key,
                    path = %path,
                    window = %e.window,
                    limit = %e.limit,
                    "Rate limit exceeded"
                );
                return Err(e.into());
            }

            debug!(client = %client_key, path = %path, "Rate limit check passed");

            // Continue to handler
            service.call(req).await
        })
    }
}

// == HELPER FUNCTIONS // ==

/// Extract client identifier from request
///
/// Priority:
/// 1. User ID from session cookie (if authenticated)
/// 2. X-Forwarded-For header (if behind proxy)
/// 3. Connection IP address
fn extract_client_key(req: &ServiceRequest) -> String {
    // Try session cookie first (for authenticated users)
    if let Some(cookie) = req.cookie("session_id") {
        return format!("session:{}", cookie.value());
    }

    // Try X-Forwarded-For (for proxied requests)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            // Take first IP in chain
            if let Some(ip) = value.split(',').next() {
                return format!("ip:{}", ip.trim());
            }
        }
    }

    // Try X-Real-IP
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            return format!("ip:{}", ip);
        }
    }

    // Fallback to connection info
    if let Some(addr) = req.connection_info().realip_remote_addr() {
        return format!("ip:{}", addr);
    }

    // Last resort
    "unknown".to_string()
}
