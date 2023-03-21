use crate::class::lens::Lens;
use crate::class::Class;
use crate::instance::Instance;
use std::any::TypeId;
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
        if lens.class == instance.class {
            Some(View {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            None
        }
    }

    pub fn read<U: 'static>(&self) -> Option<&U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                let data = self.instance.data.read().unwrap();
                Some(&*data.add(self.offset).cast::<U>())
            }
        } else {
            None
        }
    }

    pub fn write<U: 'static>(&mut self) -> Option<&mut U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                let data = self.instance.data.read().unwrap();
                Some(&mut *data.add(self.offset).cast::<U>())
            }
        } else {
            None
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
