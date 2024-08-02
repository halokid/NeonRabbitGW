use std::{future::{ready, Ready}};
use actix_http::{header, StatusCode, test};

use actix_web::{dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, http::Method, HttpResponseBuilder, HttpResponse, HttpRequest};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::http::header::{HeaderName, HeaderValue};
use futures_util::future::LocalBoxFuture;
use crate::middleware::sample2::SampleMw2;
use crate::middleware::sample::SampleMw;
//use crate::constants;

pub trait MiddleWarePl {
  fn filter(&self) -> bool;
}

pub struct MiddleWare;

impl MiddleWare {}

impl<S, B> Transform<S, ServiceRequest> for MiddleWare
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  // type Response = ServiceResponse<B>;
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Transform = MiddlewareProcess<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(MiddlewareProcess { service }))
  }
}

pub struct MiddlewareProcess<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for MiddlewareProcess<S>
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
    let fut = self.service.call(req);
    Box::pin(async move {
      // middle ware check
      let mut mws: Vec<Box<dyn MiddleWarePl>> = Vec::new();

      let sample_mw = SampleMw::new();
      mws.push(Box::new(sample_mw));

      let sample_mw2 = SampleMw2::new();
      mws.push(Box::new(sample_mw2));

      for mw in mws {
        if !mw.filter() {
          // TODO: do something fit middleware and return
        }
      }

      // use original request
      let res = fut.await?;
      Ok(res.map_body(|_head, body| EitherBody::left(body)))
    })
  }
}


