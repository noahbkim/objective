use crate::class::lens::{Lens, LensAccessor};
use crate::class::Class;
use crate::error::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct View {
    pub origin: Arc<dyn Class>,
    pub class: Arc<dyn Class>,
    pub offset: usize,
}

impl View {
    pub fn of(class: Arc<dyn Class>) -> View {
        View {
            origin: class.clone(),
            class,
            offset: 0,
        }
    }

    unsafe fn new(origin: Arc<dyn Class>, lens: Lens) -> Self {
        View {
            origin,
            class: lens.class,
            offset: lens.offset,
        }
    }

    unsafe fn access(self, lens: Lens) -> Self {
        View {
            origin: self.origin,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }
}

trait MoveViewAccessor {
    fn attr(self, name: &str) -> Result<View>;
    fn item(self, index: usize) -> Result<View>;
}

impl MoveViewAccessor for View {
    fn attr(self, name: &str) -> Result<View> {
        LensAccessor::attr(&*self.class, name).map(|lens| unsafe { self.access(lens) })
    }

    fn item(self, index: usize) -> Result<View> {
        LensAccessor::item(&*self.class, index).map(|lens| unsafe { self.access(lens) })
    }
}

impl MoveViewAccessor for Result<View> {
    fn attr(self, name: &str) -> Result<View> {
        self.and_then(|view| {
            LensAccessor::attr(&*view.class, name).map(|lens| unsafe { view.access(lens) })
        })
    }

    fn item(self, index: usize) -> Result<View> {
        self.and_then(|view| {
            LensAccessor::item(&*view.class, index).map(|lens| unsafe { view.access(lens) })
        })
    }
}

pub trait ViewAccessor {
    fn attr(&self, name: &str) -> Result<View>;
    fn item(&self, index: usize) -> Result<View>;
}

impl ViewAccessor for Arc<dyn Class> {
    fn attr(&self, name: &str) -> Result<View> {
        LensAccessor::attr(&**self, name).map(|lens| unsafe { View::new(self.clone(), lens) })
    }

    fn item(&self, index: usize) -> Result<View> {
        LensAccessor::item(&**self, index).map(|lens| unsafe { View::new(self.clone(), lens) })
    }
}

impl<T: Class + 'static> ViewAccessor for Arc<T> {
    fn attr(&self, name: &str) -> Result<View> {
        LensAccessor::attr(&**self, name).map(|lens| unsafe { View::new(self.clone(), lens) })
    }

    fn item(&self, index: usize) -> Result<View> {
        LensAccessor::item(&**self, index).map(|lens| unsafe { View::new(self.clone(), lens) })
    }
}
