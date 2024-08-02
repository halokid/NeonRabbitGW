use std::{future::{ready, Ready}};
use actix_http::Request;

use actix_web::{
	dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
	Error, http::Method, HttpResponseBuilder, HttpResponse,
};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::http::header::{HeaderName, HeaderValue};
use futures_util::future::LocalBoxFuture;
//use crate::constants;

pub struct Heartbeat;

impl<S, B> Transform<S, ServiceRequest> for Heartbeat
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  // type Response = ServiceResponse<B>;
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Transform = HeartMiddleware<S>;
  type InitError = ();
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
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  // type Response = ServiceResponse<B>;
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  dev::forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    log::debug!("-->>> procrss call by middleware");
    println!("req1 -->>> {:?}", req);
    // let new_request0 = req.request();
    // let new_request = new_request0.clone();
    // let new_request = (*new_request0).clone();
    // println!("req2 -->>> {:?}", new_request);
    // println!("kkkkkkkkkkkkkkkkk");
    // println!("req3 -->>> {:?}", req);

    // req.path() = "/ping";
    // req.

    let fut = self.service.call(req);
    Box::pin(async move {
      let res = fut.await?;
      let method = res.request().method();
      if res.request().path() == "/heartbeat-mw"
        && (method == Method::GET || method == Method::POST || method == Method::HEAD)
      {
        Ok(res.map_body(|head, _body| {
          head.headers_mut().append(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/plain"),
          );
          head.headers_mut().append(
            HeaderName::from_static("error"),
            HeaderValue::from_static("trigger heart beat"),
          );
          let box_body = BoxBody::new("heart beat middleware rsp");
					// TODO: `EitherBody::right` means  `EitherBody<B, BoxBody> use <BoxBody>`
          EitherBody::right(box_body)
        }))
      } else {
				// TODO: `EitherBody::left` means  `EitherBody<B, BoxBody> use <B>`
        Ok(res.map_body(|_head, body| EitherBody::left(body)))
      }
    })
  }
}


