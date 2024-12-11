mod server;
mod service;

use std::{future::Future, pin::Pin, time::Duration};

use server::Server;
use service::{RequestHandler, ServiceHandler};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpStream};

fn handle_request(req: httparse::Request<'_, '_>) -> std::io::Result<String> {
    let path = format!("{} {}", req.method.unwrap(), req.path.unwrap());

    if  path == "GET /" {
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: 11\r\n\r\nHello World");
        return Ok(response)
    } else {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type:text/plain\r\nContent-Length: 20\r\n\r\nERROR 404 NOT FOUND.");
        Ok(response)
    }
}

fn handler_with_timeout<'a>(req: httparse::Request<'a, 'a>) -> Pin<Box<dyn Future<Output = std::io::Result<String>> + Send + 'a>> 
{

    Box::pin(async move {
        let timeout = tokio::time::timeout(Duration::from_secs(5), async {
            handle_request(req)
        }).await;
    
        match timeout {
            Ok(Ok(res)) => Ok(res),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out"))
        }
    })
    
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let server = Server::new(3000);

    server.run(handle_request).await?;

    Ok(())
}  
