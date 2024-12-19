use std::{future::Future, marker::PhantomData, task::{ready, Poll}};

use crate::service::Service;

pub struct ReadyOneshot<T, Request> {
    inner: Option<T>,
    _p: PhantomData<Request>
}

impl <Request, T>ReadyOneshot<T, Request> 
where 
    T: Service<Request>
{
    pub fn new(service: T) -> Self {
        Self {
            inner: Some(service),
            _p: PhantomData
        }
    }
}

impl<T, Request> Unpin for ReadyOneshot<T, Request> {}

impl <T, Request> Future for ReadyOneshot<T, Request> 
where 
    T: Service<Request>,
{
    type Output = Result<T, T::Error>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        ready!(self
            .inner
            .as_mut()
            .expect("poll after Poll::Ready")
            .poll_ready(cx)
        ).unwrap();

        Poll::Ready(Ok(self.inner.take().unwrap()))


    }
}