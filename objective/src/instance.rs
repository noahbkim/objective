pub mod view;

use crate::class::Class;
use crate::error::TypeError;
use crate::instance::view::{View, Viewable};
use std::alloc::{alloc, dealloc};
use std::any::{type_name, TypeId};
use std::sync::{Arc, RwLock};

unsafe fn read<'a, U: 'static>(
    instance: &'a Instance,
    class: &'a dyn Class,
    offset: usize,
) -> Result<&'a U, TypeError> {
    if let Some(type_id) = class.id() {
        if type_id == TypeId::of::<U>() {
            unsafe {
                let data = instance.data.read().unwrap();
                // TODO: This is UB; RwLockReadGuard is needed to properly manage the state of the data
                Ok(&*data.add(offset).cast::<U>())
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast underlying type {} to {}!",
                type_name::<U>(),
                class.name(),
            )))
        }
    } else {
        Err(TypeError::new(format!(
            "Cannot cast untyped class {}!",
            class.name(),
        )))
    }
}

unsafe fn write<'a, U: 'static>(
    instance: &'a Instance,
    class: &'a dyn Class,
    offset: usize,
) -> Result<&'a mut U, TypeError> {
    // Invariant: construct() must have been called before now
    if let Some(type_id) = unsafe { class.id() } {
        if type_id == TypeId::of::<U>() {
            unsafe {
                let data = instance.data.read().unwrap();
                Ok(&mut *data.add(offset).cast::<U>())
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast underlying type {} to {}!",
                type_name::<U>(),
                class.name(),
            )))
        }
    } else {
        Err(TypeError::new(format!(
            "Cannot cast untyped class {}!",
            class.name(),
        )))
    }
}

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

    pub fn read<U: 'static>(&self) -> Result<&U, TypeError> {
        if let Some(type_id) = unsafe { self.class.id() } {
            if type_id == TypeId::of::<U>() {
                unsafe {
                    let data = self.data.read().unwrap();
                    Ok(&*data.cast::<U>())
                }
            } else {
                Err(TypeError::new(format!(
                    "Cannot cast underlying type {} to {}!",
                    type_name::<U>(),
                    self.class.name(),
                )))
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast untyped class {}!",
                self.class.name(),
            )))
        }
    }

    pub fn write<U: 'static>(&mut self) -> Result<&mut U, TypeError> {
        if let Some(type_id) = unsafe { self.class.id() } {
            if type_id == TypeId::of::<U>() {
                unsafe {
                    let data = self.data.read().unwrap();
                    Ok(&mut *data.cast::<U>())
                }
            } else {
                Err(TypeError::new(format!(
                    "Cannot cast underlying type {} to {}!",
                    type_name::<U>(),
                    self.class.name(),
                )))
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast untyped class {}!",
                self.class.name(),
            )))
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
