use std::future::{Future, ready, Ready};
use std::io;
use std::pin::Pin;
use std::str::Bytes;
use std::task::{Context, Poll};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse, Responder};
use actix_web::error::HttpError;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use tonic::Response;
use futures_util::future::LocalBoxFuture;

// pub struct MiddleWare;
pub struct MiddleWare;

impl<S, B> Transform<S, ServiceRequest> for MiddleWare
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = MiddlewareProcess<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(MiddlewareProcess {
      service
    }))
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
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    println!("Hi from start. You requested: {}", req.path());

    let fut = self.service.call(req);

    Box::pin(async move {
      // let &mut res = fut.await?;
      let mut res = fut.await?;
      // let res = fut.await?;

      //   let e= Error::
      //   res.response_mut().headers_mut().insert(
      //     header::CONTENT_TYPE,
      //     header::HeaderValue::from_static("Error"),
      // );

      // body is unchanged, map to "left" slot
      println!("Hi from response");

      // let e = actix_web::Error::into("xxx");
      // let e = HttpError::from("xxxx");
      // Err(HttpResponse::BadGateway().into())
      // Err(e)

      // res.response_mut().headers_mut().insert(
      //   header::CONTENT_TYPE,
      //   header::HeaderValue::from_static("Error"),
      // );

      // rbj.msg = format!("{}. I modified the request.", rbj.msg);
      // let new_rbj_result = serde_json::to_string(&rbj);
      // let new_rbj_str = new_rbj_result.unwrap();
      // let body_final = Bytes::from(new_rbj_str);

      let str = "Hello, world!";
let bytes = str.as_bytes();
      // res.response_mut().as_ref().set_body(bytes);
      res.response_mut().set_body(bytes);
      // let resx = res;

      Ok(res)
      // return Err()
      // Err(e)
    })
  }
}


