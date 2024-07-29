use std::future::{Future, ready, Ready};
use std::task::{Context, Poll};
use actix_web::{
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error,
};
use tonic::Response;
use futures_util::future::LocalBoxFuture;

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
      let res = fut.await?;
      println!("Hi from response");
      Ok(res)
    })
  }
}


