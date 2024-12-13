use builder::{ServiceBuilder, Stack};
use layer::Layer;
use service::{RequestHandlerLayer, Service, ServiceHandler};

mod service;
mod layer;
mod builder;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let stack = ServiceBuilder::new().layer(RequestHandlerLayer).service(ServiceHandler);



    Ok(())
}  
