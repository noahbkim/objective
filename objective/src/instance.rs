pub mod view;

use crate::class::Class;
use crate::class::view::View;
use crate::error::{Error, Result};
use crate::instance::view::{ReadView, ReadViewable, WriteView, WriteViewable};
use std::alloc::{alloc, dealloc};
use std::any::{type_name, TypeId};
use std::borrow::Borrow;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

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

    pub fn read(&self) -> std::result::Result<InstanceReadGuard, PoisonError<InstanceReadGuard>> {
        match self.data.read() {
            Ok(data) => Ok(InstanceReadGuard {
                class: self.class.clone(),
                data,
            }),
            Err(error) => Err(PoisonError::new(InstanceReadGuard {
                class: self.class.clone(),
                data: error.into_inner(),
            })),
        }
    }

    pub fn write(&self) -> std::result::Result<InstanceWriteGuard, PoisonError<InstanceWriteGuard>> {
        match self.data.write() {
            Ok(data) => Ok(InstanceWriteGuard {
                class: self.class.clone(),
                data,
            }),
            Err(error) => Err(PoisonError::new(InstanceWriteGuard {
                class: self.class.clone(),
                data: error.into_inner(),
            })),
        }
    }
}

pub struct InstanceReadGuard<'a> {
    class: Arc<dyn Class>,
    data: RwLockReadGuard<'a, *mut u8>,
}

impl<'a> InstanceReadGuard<'a> {
    unsafe fn cast_at<'b, 'c: 'a, U: 'static>(&self, class: &'b dyn Class, offset: usize) -> Result<&'c U> {
        if let Some(type_id) = class.value() {
            if type_id == TypeId::of::<U>() {
                Ok(&*self.data.add(offset).cast::<U>())
            } else {
                Err(Error::ValueError(format!(
                    "Cannot cast underlying type {} to {:?}!",
                    type_name::<U>(),
                    class,
                )))
            }
        } else {
            Err(Error::TypeError(format!("Cannot cast untyped class {:?}!", class)))
        }
    }

    pub fn cast<U: 'static>(&self) -> Result<&U> {
        unsafe {
            self.cast_at(self.class.borrow(), 0)
        }
    }

    pub fn attr<'b>(&'b self, name: &str) -> Result<ReadView<'a, 'b>> {
        ReadView::of(self).attr(name)
    }

    pub fn item<'b>(&'b self, index: usize) -> Result<ReadView<'a, 'b>> {
        ReadView::of(self).item(index)
    }

    pub fn through<'b>(&'b self, lens: &View) -> Result<ReadView<'a, 'b>> {
        ReadView::apply(lens, self)
    }
}

pub struct InstanceWriteGuard<'a> {
    class: Arc<dyn Class>,
    data: RwLockWriteGuard<'a, *mut u8>,
}

impl<'a> InstanceWriteGuard<'a> {
    unsafe fn cast_at<'b, 'c: 'a, U: 'static>(&'c self, class: &'b dyn Class, offset: usize) -> Result<&'c mut U> {
        if let Some(type_id) = class.value() {
            if type_id == TypeId::of::<U>() {
                Ok(&mut *self.data.add(offset).cast::<U>())
            } else {
                Err(Error::ValueError(format!(
                    "Cannot cast underlying type {} to {:?}!",
                    type_name::<U>(),
                    class,
                )))
            }
        } else {
            Err(Error::TypeError(format!("Cannot cast untyped class {:?}!", class)))
        }
    }

    pub fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe {
            self.cast_at(self.class.borrow(), 0)
        }
    }

    pub fn attr<'b>(&'b self, name: &str) -> Result<WriteView<'a, 'b>> {
        WriteView::of(self).attr(name)
    }

    pub fn item<'b>(&'b self, index: usize) -> Result<WriteView<'a, 'b>> {
        WriteView::of(self).item(index)
    }

    pub fn through<'b>(&'b self, lens: &View) -> Result<WriteView<'a, 'b>> {
        WriteView::apply(lens, self)
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
