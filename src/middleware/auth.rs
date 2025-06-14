use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use crate::{config::Config, utils::verify_token};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let config = req.app_data::<Config>().unwrap();
            
            if let Some(auth_header) = req.headers().get("Authorization") {
                let auth_str = auth_header.to_str().unwrap_or("");
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    if let Ok(claims) = verify_token(token, config) {
                        req.extensions_mut().insert(claims);
                        return service.call(req).await;
                    }
                }
            }
            
            Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        })
    }
} 