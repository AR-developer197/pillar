use builder::ServiceBuilder;
use service::{RequestHandler2, RequestHandlerLayer, Service, ServiceHandler};

mod service;
mod layer;
mod builder;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    let mut stack= ServiceBuilder::new()
        .layer(RequestHandlerLayer)
        .layer_fn(RequestHandler2::new)
        .service(ServiceHandler);
    
    stack.call("req").await?;

    Ok(())
}  
