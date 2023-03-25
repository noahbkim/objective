use crate::accessor::{Accessor, IntoAccessor, MutableCast};
use crate::class::lens::Lens;
use crate::class::view::View;
use crate::class::Class;
use crate::error::{Error, Result};
use crate::instance::Instance;
use std::any::{type_name, TypeId};
use std::borrow::Borrow;
use std::sync::{Arc, PoisonError, RwLockWriteGuard};

pub struct InstanceWriteGuard<'g> {
    class: Arc<dyn Class>,
    data: RwLockWriteGuard<'g, *mut u8>,
}

impl<'g> InstanceWriteGuard<'g> {
    pub fn acquire(instance: &'g Instance) -> std::result::Result<Self, PoisonError<Self>> {
        match instance.data.write() {
            Ok(data) => Ok(Self {
                class: instance.class.clone(),
                data,
            }),
            Err(error) => Err(PoisonError::new(Self {
                class: instance.class.clone(),
                data: error.into_inner(),
            })),
        }
    }

    unsafe fn cast<U: 'static>(&self, class: &dyn Class, offset: usize) -> Result<&'g mut U> {
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
            Err(Error::TypeError(format!(
                "Cannot cast untyped class {:?}!",
                class
            )))
        }
    }

    pub fn attr(&self, name: &str) -> Result<WriteReference<'_>> {
        WriteReference::of(self).attr(name)
    }

    pub fn item(&self, index: usize) -> Result<WriteReference<'_>> {
        WriteReference::of(self).item(index)
    }

    pub fn through(&self, lens: &View) -> Result<WriteReference<'_>> {
        WriteReference::apply(lens, self)
    }
}

unsafe impl<'g> MutableCast for InstanceWriteGuard<'g> {
    fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe { self.cast(self.class.borrow(), 0) }
    }
}

#[derive(Clone)]
pub struct WriteReference<'g> {
    instance: &'g InstanceWriteGuard<'g>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl<'g> WriteReference<'g> {
    pub fn of(instance: &'g InstanceWriteGuard<'g>) -> Self {
        WriteReference {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn apply(lens: &View, instance: &'g InstanceWriteGuard<'g>) -> Result<Self> {
        if lens.origin.id() == instance.class.id() {
            Ok(WriteReference {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            Err(Error::TypeError(format!(
                "View of type {:?} cannot be applied to instance of type {:?}",
                lens.origin, instance.class
            )))
        }
    }

    unsafe fn access(self, lens: Lens) -> Self {
        WriteReference {
            instance: &self.instance,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }
}

unsafe impl<'g> MutableCast for WriteReference<'g> {
    fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe { self.instance.cast(self.class.borrow(), self.offset) }
    }
}

unsafe impl<'g> IntoAccessor<WriteReference<'g>> for WriteReference<'g> {
    fn attr(self, name: &str) -> Result<WriteReference<'g>> {
        Accessor::<Lens>::attr(&*self.class, name).map(|lens| unsafe { self.access(lens) })
    }

    fn item(self, index: usize) -> Result<WriteReference<'g>> {
        Accessor::<Lens>::item(&*self.class, index).map(|lens| unsafe { self.access(lens) })
    }
}

unsafe impl<'g> IntoAccessor<WriteReference<'g>> for Result<WriteReference<'g>> {
    fn attr(self, name: &str) -> Result<WriteReference<'g>> {
        self.and_then(|reference| reference.attr(name))
    }

    fn item(self, index: usize) -> Result<WriteReference<'g>> {
        self.and_then(|reference| reference.item(index))
    }
}
