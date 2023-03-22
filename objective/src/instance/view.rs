use crate::class::lens::Lens;
use crate::class::Class;
use crate::error::TypeError;
use crate::instance::{read, write, Instance};
use std::borrow::Borrow;
use std::sync::Arc;

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

    pub fn apply(lens: &Lens, instance: Arc<Instance>) -> Option<View> {
        if std::ptr::eq(lens.origin.as_ref(), instance.class.as_ref()) {
            Some(View {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            None
        }
    }

    pub fn read<U: 'static>(&self) -> Result<&U, TypeError> {
        unsafe {
            read(self.instance.borrow(), self.class.borrow(), self.offset)
        }
    }

    pub fn write<U: 'static>(&mut self) -> Result<&mut U, TypeError> {
        unsafe {
            write(self.instance.borrow(), self.class.borrow(), self.offset)
        }
    }
}

pub trait Viewable {
    fn attr(self, name: &str) -> Option<View>;
    fn item(self, index: usize) -> Option<View>;
}

impl Viewable for View {
    fn attr(self, name: &str) -> Option<View> {
        self.class.attr(name).map(|part| self.zoom(part))
    }

    fn item(self, index: usize) -> Option<View> {
        self.class.item(index).map(|part| self.zoom(part))
    }
}

impl Viewable for Option<View> {
    fn attr(self, name: &str) -> Option<View> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Option<View> {
        self.and_then(|view| view.item(index))
    }
}
