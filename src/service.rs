use std::{fmt::Debug, future::Future, pin::Pin};

pub struct ServiceHandler;

pub trait Service<T> {
    type Response;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    type Error;

    fn call(&mut self, req: T) -> Self::Future;
}

pub struct RequestHandler<S> {
    inner: S
}

impl <S>RequestHandler<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
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
        println!("req: {:#?}", req);

        let response = self.inner.call(req);

        Box::pin(async move{
            let response = response.await?;
            let response = format!("{:#?}", response);
            Ok(response)
        })
    }
}

impl <T> Service<T> for ServiceHandler {
    type Response = String;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    type Error = std::io::Error;

    fn call(&mut self, _: T) -> Self::Future{

        Box::pin(async{
            Ok("hello".to_string())
        })
    }
}