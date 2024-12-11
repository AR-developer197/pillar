use std::{future::Future, net::SocketAddr, pin::Pin};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use crate::service::{RequestHandler, Service, ServiceHandler};

pub struct Server {
    addr: u16
}

impl Server {
    pub fn new(addr: u16) -> Self {
        Self {
            addr
        }
    }

    pub async fn run<T>(&self, handler: T) -> std::io::Result<()> 
    where 
        T: Fn(httparse::Request<'_, '_>) -> std::io::Result<String> + Send + Copy + 'static,
    {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.addr));

        let connection = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = connection.accept().await?;
            
            
            
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer).await {
                    Ok(_) => println!("ye"),
                    Err(_) => println!("fck"),
                };

                let mut headers = [httparse::EMPTY_HEADER; 16];
                let mut req= httparse::Request::new(&mut headers);
                req.parse(&buffer).unwrap();
                 

                let mut sve = RequestHandler::new(ServiceHandler);

                let sve = sve.call(&req).await.unwrap();

                println!("{}", sve);

                let response = handler(req).unwrap();
                
                stream.write_all(response.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            });
        }
    }
}

