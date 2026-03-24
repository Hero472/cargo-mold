use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::common::errors::AppError;

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = AppError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = web::Json::<T>::from_request(req, payload);
        Box::pin(async move {
            let json = fut.await
                .map_err(|e| AppError::BadRequest(e.to_string()))?;

            json.validate()
                .map_err(|e| AppError::BadRequest(e.to_string()))?;

            Ok(ValidatedJson(json.into_inner()))
        })
    }
}