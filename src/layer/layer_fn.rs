use super::Layer;

#[derive(Debug)]
pub struct LayerFn<F> {
    f: F
}

pub fn layer_fn<F>(f: F) -> LayerFn<F> 
{
    LayerFn { f }
}

impl <F, S, Out>Layer<S> for LayerFn<F>
where 
    F: Fn(S) -> Out
{
    type Service = Out;

    fn layer(&self, inner: S) -> Self::Service 
    {
        (self.f)(inner)
    }
}