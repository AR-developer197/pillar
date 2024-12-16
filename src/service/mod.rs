use std::{fmt::Debug, future::Future, pin::Pin};

use crate::layer::Layer;

pub trait Service<T> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: T) -> Self::Future;
}

#[derive(Debug)]
pub struct RequestHandler<S> {
    inner: S
}

impl <S>RequestHandler<S> {
    pub fn _new(inner: S) -> Self{
        Self {inner}
    }
}

impl <T, S> Service<T> for RequestHandler<S>
where 
    T: Debug,
    S: Service<T> + Send,
    <S as Service<T>>::Future: Send + 'static,
    <S as Service<T>>::Response: Debug,
    std::io::Error: From <<S as Service<T>>::Error>
    
{
    type Response = String;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    type Error = std::io::Error;


    fn call(&mut self, req: T) -> Self::Future{
        println!("1st req: {:#?}", req);

        let response = self.inner.call(req);

        Box::pin(async move{
            let response = response.await?;
            let response = format!("{:#?}", response);
            Ok(response)
        })
    }
}

#[derive(Debug)]
pub struct ServiceHandler;

impl <T> Service<T> for ServiceHandler {
    type Response = String;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    type Error = std::io::Error;

    fn call(&mut self, _: T) -> Self::Future{

        println!("final");

        Box::pin(async{
            Ok("hello".to_string())
        })
    }
}

#[derive(Debug)]
pub struct RequestHandlerLayer;

impl <S> Layer<S> for RequestHandlerLayer {
    type Service = RequestHandler<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestHandler { inner }
    }
}

pub struct RequestHandler2<S> {
    inner: S
}

impl <S>RequestHandler2<S> {
    pub fn new(inner: S) -> Self {
        RequestHandler2 {inner}
    }
}

impl <T, S> Service<T> for RequestHandler2<S>
where 
    T: Debug,
    S: Service<T> + Send,
    <S as Service<T>>::Future: Send + 'static,
    <S as Service<T>>::Response: Debug,
    std::io::Error: From <<S as Service<T>>::Error>
    
{
    type Response = String;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    type Error = std::io::Error;


    fn call(&mut self, req: T) -> Self::Future{
        println!("2nd req: {:#?}", req);

        let response = self.inner.call(req);

        Box::pin(async move{
            let response = response.await?;
            let response = format!("{:#?}", response);
            Ok(response)
        })
    }
}

#[derive(Debug)]
pub struct RequestHandlerLayer2;

impl <S> Layer<S> for RequestHandlerLayer2 {
    type Service = RequestHandler2<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestHandler2 { inner }
    }
}