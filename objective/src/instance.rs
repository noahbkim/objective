pub mod view;

use crate::class::Class;
use crate::instance::view::{View, Viewable};
use std::alloc::{alloc, dealloc};
use std::any::TypeId;
use std::sync::{Arc, RwLock};

pub struct Instance {
    class: Arc<dyn Class>,
    data: RwLock<*mut u8>,
}

impl Instance {
    pub fn new(class: Arc<dyn Class>) -> Self {
        // Invariant: construct expects to have at least size() data
        // Must be deallocated in drop
        unsafe {
            let data = alloc(class.layout());
            class.construct(data);
            Self {
                class,
                data: RwLock::new(data),
            }
        }
    }

    pub fn read<U: 'static>(&self) -> Option<&U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                let data = self.data.read().unwrap();
                Some(&*data.cast::<U>())
            }
        } else {
            None
        }
    }

    pub fn write<U: 'static>(&mut self) -> Option<&mut U> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                let data = self.data.write().unwrap();
                Some(&mut *data.cast::<U>())
            }
        } else {
            None
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // Invariant: is not null, has layout of self.class.layout()
        unsafe {
            dealloc(*self.data.write().unwrap(), self.class.layout());
        }
    }
}

impl Viewable for Arc<Instance> {
    fn attr(self, name: &str) -> Option<View> {
        View::of(self).attr(name)
    }

    fn item(self, index: usize) -> Option<View> {
        View::of(self).item(index)
    }
}
