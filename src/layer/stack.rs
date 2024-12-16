use super::Layer;

#[derive(Debug)]
pub struct Stack<Inner, Outer> {
    inner: Inner,
    outer: Outer
}

impl <Inner, Outer> Stack<Inner, Outer> {
    pub fn new(inner: Inner, outer: Outer) -> Self {
        Self {inner, outer}
    }
}

impl <S, Inner, Outer> Layer<S> for Stack<Inner, Outer> 
where 
    Inner: Layer<S>,
    Outer: Layer<Inner::Service>
{
    type Service = Outer::Service;

    fn layer(&self, service: S) -> Self::Service {
        let inner = self.inner.layer(service);
        
        self.outer.layer(inner)
    }
}