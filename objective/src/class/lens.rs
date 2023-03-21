use crate::class::Class;
use crate::instance::Instance;

use std::sync::Arc;
use crate::instance::view::View;

#[derive(Clone)]
pub struct Lens {
    pub origin: Arc<dyn Class>,
    pub class: Arc<dyn Class>,
    pub offset: usize,
}

impl Lens {
    pub fn of(class: Arc<dyn Class>) -> Lens {
        Lens {
            origin: class.clone(),
            class,
            offset: 0,
        }
    }

    fn zoom(self, part: (Arc<dyn Class>, usize)) -> Lens {
        Lens {
            origin: self.origin,
            class: part.0,
            offset: self.offset + part.1,
        }
    }

    // TODO: result
    pub fn apply(&self, instance: Instance) -> Option<View> {
        if instance.class == self.origin {
            Some(View {
                instance,
                class: self.class,
                offset: self.offset,
            })
        } else {
            None
        }
    }
}

trait Focal {
    fn attr(self, name: &str) -> Option<Lens>;
    fn item(self, index: usize) -> Option<Lens>;
}

impl Focal for Lens {
    fn attr(self, name: &str) -> Option<Lens> {
        self.class.attr(name).map(|part| self.zoom(part))
    }

    fn item(self, index: usize) -> Option<Lens> {
        self.class.item(index).map(|part| self.zoom(part))
    }
}
