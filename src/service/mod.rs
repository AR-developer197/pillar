use std::{future::Future, task::{Context, Poll}};

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx:  &mut Context<'_>) -> Poll<Result<(), std::io::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}

