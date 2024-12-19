mod ready;

use ready::ReadyOneshot;

use crate::service::Service;

pub trait ServiceExt<Request>: Service<Request> {

    async fn ready(self) -> Result<Self, <Self as Service<Request>>::Error>
    where Self: Sized
    {
        ReadyOneshot::new(self).await
    }

}

impl<T, Request> ServiceExt<Request> for T where T: Service<Request> {}