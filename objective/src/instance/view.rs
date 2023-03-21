use std::any::TypeId;
use crate::class::Class;
use crate::instance::Instance;

use std::sync::Arc;

#[derive(Clone)]
pub struct View {
    instance: Arc<Instance>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl View {
    pub fn new(instance: Arc<Instance>) -> View {
        View {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn downcast_ref<U: 'static>(&self) -> Option<&U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                Some(&*self.data.read().unwrap().cast::<U>())
            }
        } else {
            None
        }
    }

    pub fn downcast_mut<U: 'static>(&mut self) -> Option<&mut U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                Some(&mut *self.data.write().unwrap().cast::<U>())
            }
        } else {
            None
        }
    }
}

pub trait Viewable {
    fn view(&self) -> View;
    fn attr(self, name: &String) -> Option<View>;
    fn item(self, index: usize) -> Option<View>;
}

impl Viewable for View {
    fn view(&self) -> View {
        self.clone()
    }

    fn attr(self, name: &String) -> Option<View> {
        if let Some((class, offset)) = self.class.attr(name) {
            Some(View {
                instance: self.instance.clone(),
                class,
                offset: self.offset + offset,
            })
        } else {
            None
        }
    }

    fn item(self, index: usize) -> Option<View> {
        todo!()
    }
}

