use crate::class::lens::Lens;
use crate::class::Class;
use crate::instance::{Instance, Read, Write};
use crate::error::{Error, Result};
use std::sync::{Arc, PoisonError};

#[derive(Clone)]
pub struct View {
    instance: Arc<Instance>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl View {
    pub fn of(instance: Arc<Instance>) -> View {
        View {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    fn zoom(self, part: (Arc<dyn Class>, usize)) -> View {
        View {
            instance: self.instance,
            class: part.0,
            offset: self.offset + part.1,
        }
    }

    pub fn apply(lens: &Lens, instance: Arc<Instance>) -> Result<View> {
        if std::ptr::eq(lens.origin.as_ref(), instance.class.as_ref()) {
            Ok(View {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            Err(Error::TypeError(format!(
                "View of type {} cannot be applied to instance of type {}",
                lens.origin.name(),
                instance.class.name()
            )))
        }
    }

    pub fn read(&self) -> std::result::Result<Read, PoisonError<Read>> {
        unsafe {
            self.instance.read_at(self.class.clone(), self.offset)
        }
    }

    pub fn write(&mut self) -> std::result::Result<Write, PoisonError<Write>> {
        unsafe {
            self.instance.write_at(self.class.clone(), self.offset)
        }
    }
}

pub trait Viewable {
    fn attr(self, name: &str) -> Result<View>;
    fn item(self, index: usize) -> Result<View>;
}

impl Viewable for View {
    fn attr(self, name: &str) -> Result<View> {
        self.class.attr(name).map(|part| self.zoom(part))
    }

    fn item(self, index: usize) -> Result<View> {
        self.class.item(index).map(|part| self.zoom(part))
    }
}

impl Viewable for Result<View> {
    fn attr(self, name: &str) -> Result<View> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Result<View> {
        self.and_then(|view| view.item(index))
    }
}
