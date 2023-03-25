use crate::class::Class;
use std::sync::Arc;

pub struct Lens {
    pub class: Arc<dyn Class>,
    pub offset: usize,
}
