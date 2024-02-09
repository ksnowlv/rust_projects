use std::future::{ready, Ready};

use actix_http::{
	body::{BoxBody, EitherBody},
	header::{self, HeaderName, HeaderValue},
};
use actix_web::{
	dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
	http::Method,
	Error, HttpResponse, HttpResponseBuilder,web::JsonBody,
};

use actix_http::body::to_bytes;

use futures_util::future::LocalBoxFuture;
use log::debug;
//use crate::constants;

use hyper::body;

pub struct Heartbeat;

impl<S, B> Transform<S, ServiceRequest> for Heartbeat
	where
		S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Response = ServiceResponse<EitherBody<B, BoxBody>>;
	type Error = Error;
	type InitError = ();
	type Transform = HeartMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(HeartMiddleware { service }))
	}
}

pub struct HeartMiddleware<S> {
	service: S,
}

impl<S, B> Service<ServiceRequest> for HeartMiddleware<S>
	where
		S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Response = ServiceResponse<EitherBody<B, BoxBody>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	dev::forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {

		let fut = self.service.call(req);
		Box::pin(async move {
			let res = fut.await?;
			let method = res.request().method();
			if res.request().path() == "/ping"
				&& (method == Method::GET || method == Method::POST || method == Method::HEAD)
			{
				Ok(res.map_body(|head, _body| {
					head.headers_mut().append(
						HeaderName::from_static("content-type"),
						HeaderValue::from_static("text/plain"),
					);
					let box_body = BoxBody::new("body test");
					EitherBody::right(box_body)
				}))
			} else {
				let status = res.status();
				let headers = res.headers().clone();

				debug!("Status Code: {}", status);
				debug!("Headers: {:?}", headers);

//				let body_bytes = to_bytes(res.into_body()).await?;
//				let body = String::from_utf8(body_bytes.to_vec())?;

				// Convert body to string and print
//				let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
//				let body_string = String::from_utf8_lossy(&body_bytes);
//				println!("Response Body as String: {}", body_string);
//				let (parts, bod) = res.into_parts();
//
//				let bytes = actix_http::body::to_bytes(bod).await?;
//				println!("{:?}", bytes);

				//Response::from_parts(parts, actix_http::body);


				Ok(res.map_body(|_head, body| EitherBody::left({ body })))
			}
		})
	}
}
