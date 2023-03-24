pub mod view;

use crate::class::Class;
use crate::error::TypeError;
use crate::instance::view::{View, Viewable};
use std::alloc::{alloc, dealloc};
use std::any::{type_name, TypeId};
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Read<'a> {
    class: Arc<dyn Class>,
    guard: RwLockReadGuard<'a, *mut u8>,
    offset: usize,
}

impl<'a> Read<'a> {
    pub fn cast<U: 'static>(&self) -> Result<&U, TypeError> {
        if let Some(type_id) = unsafe { self.class.id() } {
            if type_id == TypeId::of::<U>() {
                unsafe {
                    Ok(&*self.guard.add(self.offset).cast::<U>())
                }
            } else {
                Err(TypeError::new(format!(
                    "Cannot cast underlying type {} to {}!", type_name::<U>(), self.class.name(),
                )))
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast untyped class {}!", self.class.name()
            )))
        }
    }
}

pub struct Write<'a>{
    class: Arc<dyn Class>,
    guard: RwLockWriteGuard<'a, *mut u8>,
    offset: usize,
}

impl<'a> Write<'a> {
    pub fn cast<U: 'static>(&self) -> Result<&mut U, TypeError> {
        if let Some(type_id) = unsafe { self.class.id() } {
            if type_id == TypeId::of::<U>() {
                unsafe {
                    Ok(&mut *self.guard.add(self.offset).cast::<U>())
                }
            } else {
                Err(TypeError::new(format!(
                    "Cannot cast underlying type {} to {}!", type_name::<U>(), self.class.name(),
                )))
            }
        } else {
            Err(TypeError::new(format!(
                "Cannot cast untyped class {}!", self.class.name()
            )))
        }
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

    unsafe fn read_at(
        &self,
        class: Arc<dyn Class>,
        offset: usize,
    ) -> Result<Read, PoisonError<Read>> {
        match self.data.read() {
            Ok(guard) => Ok(Read { class, guard, offset }),
            Err(error) => Err(PoisonError::new(Read {
                class,
                guard: error.into_inner(),
                offset,
            })),
        }
    }

    pub fn read(&self) -> Result<Read, PoisonError<Read>> {
        unsafe {
            self.read_at(self.class.clone(), 0)
        }
    }

    unsafe fn write_at(
        &self,
        class: Arc<dyn Class>,
        offset: usize,
    ) -> Result<Write, PoisonError<Write>> {
        match self.data.write() {
            Ok(guard) => Ok(Write { class, guard, offset }),
            Err(error) => Err(PoisonError::new(Write {
                class,
                guard: error.into_inner(),
                offset,
            })),
        }
    }

    pub fn write(&self) -> Result<Write, PoisonError<Write>> {
        unsafe {
            self.write_at(self.class.clone(), 0)
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
