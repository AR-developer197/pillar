use super::layer::Layer;

#[derive(Debug)]
pub struct Identity {
    _p: (),
}

impl Identity {
    pub fn new() -> Self{
        Identity { _p: ()}
    }
}

impl <S>Layer<S> for Identity {
    type Service = S;

    fn layer(&self, inner: S) -> Self::Service {
        inner
    }
}