use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::time::Instant;

pub struct ApiInterceptor;

impl<S, B> Transform<S, ServiceRequest> for ApiInterceptor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InterceptorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(InterceptorMiddleware { service })
    }
}

pub struct InterceptorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for InterceptorMiddleware<S>
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
        let start_time = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let elapsed = start_time.elapsed().as_millis();

            // Inject custom headers into the final response
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-response-time-ms"),
                actix_web::http::header::HeaderValue::from_str(&elapsed.to_string()).unwrap(),
            );

            println!("[API] {} {} - {}ms", method, path, elapsed);
            Ok(res)
        })
    }
}