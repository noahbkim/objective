use crate::class::Class;
use crate::error::{Error, Result};
use std::fmt::Debug;
use std::sync::Arc;

pub struct Lens {
    pub class: Arc<dyn Class>,
    pub offset: usize,
}

pub unsafe trait LensAccessor: Debug {
    fn attr(&self, _name: &str) -> Result<Lens> {
        Err(Error::TypeError(format!("Class {:?} does not support attribute access!", self)))
    }

    fn item(&self, _index: usize) -> Result<Lens> {
        Err(Error::TypeError(format!("Class {:?} does not support index access!", self)))
    }
}
