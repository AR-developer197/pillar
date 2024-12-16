use core::fmt;

use crate::layer::{layer_fn, Identity, Layer, LayerFn, Stack};

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

    pub fn layer_fn<F>(self, inner: F) -> ServiceBuilder<Stack<LayerFn<F>, L>>
    where 
        L: fmt::Debug
    {
        println!("{:#?}", &self);
        self.layer(layer_fn(inner))
    }

    pub fn service<S>(self, serivce: S) -> L::Service
    where 
        L: Layer<S>
    {
        self.layer.layer(serivce)
    }
}