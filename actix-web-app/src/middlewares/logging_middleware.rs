use std::future::{ready, Ready};
use std::io::Bytes;
use actix_http::body::{MessageBody, to_bytes};
use actix_http::{HttpMessage, StatusCode};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse, web};
use futures_util::future::LocalBoxFuture;
use futures_util::{FutureExt, StreamExt};
use log::{info, warn, debug};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Logging;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Logging
	where
		S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = LoggingMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(LoggingMiddleware { service }))
	}
}

pub struct LoggingMiddleware<S> {
	service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
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
		let path = req.path().to_string();
		let headers = req.headers().to_owned();
		let query_params = req.query_string().to_string();

		debug!("request path: {}, \nheader:{:?}, \nquery_params:{} ",
			path, headers, query_params);

		let future = self.service.call(req);

		Box::pin(async move {

			let res: ServiceResponse<B> = future.await?;

			let status = res.status();
			let headers = res.headers().clone();
			//let bodys = res.body();
			let res_data = res.response().body().clone();

			debug!("Status Code: {}", status);
			debug!("Headers: {:?}", headers);
			Ok(res)
		})
	}
}
