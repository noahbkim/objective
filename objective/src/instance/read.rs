use crate::accessor::{Accessor, Cast, IntoAccessor};
use crate::class::lens::Lens;
use crate::class::view::View;
use crate::class::Class;
use crate::error::{Error, Result};
use crate::instance::Instance;
use std::any::{type_name, TypeId};
use std::borrow::Borrow;
use std::sync::{Arc, PoisonError, RwLockReadGuard};

pub struct InstanceReadGuard<'g> {
    class: Arc<dyn Class>,
    data: RwLockReadGuard<'g, *mut u8>,
}

impl<'g> InstanceReadGuard<'g> {
    pub fn acquire(instance: &'g Instance) -> std::result::Result<Self, PoisonError<Self>> {
        match instance.data.read() {
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

    unsafe fn cast<U: 'static>(&self, class: &dyn Class, offset: usize) -> Result<&'g U> {
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
            Err(Error::TypeError(format!(
                "Cannot cast untyped class {:?}!",
                class
            )))
        }
    }

    pub fn attr(&self, name: &str) -> Result<ReadReference<'_>> {
        ReadReference::of(self).attr(name)
    }

    pub fn item(&self, index: usize) -> Result<ReadReference<'_>> {
        ReadReference::of(self).item(index)
    }

    pub fn through(&self, lens: &View) -> Result<ReadReference<'_>> {
        ReadReference::apply(lens, self)
    }
}

unsafe impl<'g> Cast for InstanceReadGuard<'g> {
    fn cast<U: 'static>(&self) -> Result<&U> {
        unsafe { self.cast(self.class.borrow(), 0) }
    }
}

#[derive(Clone)]
pub struct ReadReference<'g> {
    instance: &'g InstanceReadGuard<'g>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl<'g> ReadReference<'g> {
    pub fn of(instance: &'g InstanceReadGuard<'g>) -> Self {
        ReadReference {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn apply(lens: &View, instance: &'g InstanceReadGuard<'g>) -> Result<Self> {
        if lens.origin.id() == instance.class.id() {
            Ok(ReadReference {
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
        ReadReference {
            instance: &self.instance,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }
}

unsafe impl<'g> Cast for ReadReference<'g> {
    fn cast<U: 'static>(&self) -> Result<&U> {
        unsafe { self.instance.cast(self.class.borrow(), self.offset) }
    }
}

unsafe impl<'g> IntoAccessor<ReadReference<'g>> for ReadReference<'g> {
    fn attr(self, name: &str) -> Result<Self> {
        Accessor::<Lens>::attr(&*self.class, name).map(|lens| unsafe { self.access(lens) })
    }

    fn item(self, index: usize) -> Result<ReadReference<'g>> {
        Accessor::<Lens>::item(&*self.class, index).map(|lens| unsafe { self.access(lens) })
    }
}

unsafe impl<'g> IntoAccessor<ReadReference<'g>> for Result<ReadReference<'g>> {
    fn attr(self, name: &str) -> Result<ReadReference<'g>> {
        self.and_then(|reference| reference.attr(name))
    }

    fn item(self, index: usize) -> Result<ReadReference<'g>> {
        self.and_then(|reference| reference.item(index))
    }
}
