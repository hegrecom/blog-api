use std::future::{Ready, ready};

use actix_web::dev::{Transform, ServiceRequest, Service, ServiceResponse, self};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationService { service }))
    }
}

pub struct AuthenticationService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

