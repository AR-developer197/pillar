use std::{future::Future, os::raw, pin::Pin, process::Output, task::Poll};

use service::Service;
use util::ServiceExt;

mod service;
mod layer;
mod builder;
mod util;

struct ServiceHandlder;

impl Service<String> for ServiceHandlder {
    type Response = String;

    type Error = std::io::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx:  &mut std::task::Context<'_>) -> std::task::Poll<Result<(), std::io::Error>> {
        println!("hi");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        

        Box::pin(async move {
            Ok(req)
        })
    }

}

#[tokio::main]
async fn main() -> std::io::Result<()>{ 

    let s = ServiceHandlder.ready().await?;

    Ok(())
}  
