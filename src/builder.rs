use crate::layer::Layer;

#[derive(Debug)]
pub struct Identity {
    _p: (),
}

impl Identity {
    fn new() -> Self{
        Identity { _p: ()}
    }
}

impl <S>Layer<S> for Identity {
    type Service = S;

    fn layer(&self, inner: S) -> Self::Service {
        inner
    }
}


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

#[derive(Debug)]
pub struct ServiceBuilder<L> {
    layer: L
}

impl ServiceBuilder<Identity> {
    pub fn new() -> Self {
        Self { layer: Identity::new() }
    }
}

impl <L> ServiceBuilder<L> {
    pub fn layer<S>(self, inner: S) -> ServiceBuilder<Stack<S, L>>
    {
        ServiceBuilder { layer: Stack::new(inner, self.layer) }
    }

    pub fn service<S>(self, serivce: S) -> L::Service
    where 
        L: Layer<S>
    {
        self.layer.layer(serivce)
    }
}