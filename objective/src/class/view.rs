use crate::accessor::{Accessor, IntoAccessor};
use crate::class::lens::Lens;
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

    unsafe fn access(self, lens: Lens) -> Self {
        View {
            origin: self.origin,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }
}

unsafe impl IntoAccessor<View> for View {
    fn attr(self, name: &str) -> Result<View> {
        Accessor::<Lens>::attr(&*self.class, name).map(|lens| unsafe { self.access(lens) })
    }

    fn item(self, index: usize) -> Result<View> {
        Accessor::<Lens>::item(&*self.class, index).map(|lens| unsafe { self.access(lens) })
    }
}

unsafe impl IntoAccessor<View> for Result<View> {
    fn attr(self, name: &str) -> Result<View> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Result<View> {
        self.and_then(|view| view.item(index))
    }
}

unsafe impl Accessor<View> for Arc<dyn Class> {
    fn attr(&self, name: &str) -> Result<View> {
        View::of(self.clone()).attr(name)
    }

    fn item(&self, index: usize) -> Result<View> {
        View::of(self.clone()).item(index)
    }
}

unsafe impl<T: Class + 'static> Accessor<View> for Arc<T> {
    fn attr(&self, name: &str) -> Result<View> {
        View::of(self.clone()).attr(name)
    }

    fn item(&self, index: usize) -> Result<View> {
        View::of(self.clone()).item(index)
    }
}
