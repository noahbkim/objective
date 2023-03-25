use crate::accessor::{Accessor, IntoAccessor};
use crate::class::lens::Lens;
use crate::class::view::View;
use crate::class::Class;
use crate::error::{Error, Result};
use std::any::{type_name, TypeId};
use std::borrow::Borrow;
use std::sync::{Arc, RwLockWriteGuard};

pub struct InstanceWriteGuard<'g> {
    class: Arc<dyn Class>,
    data: RwLockWriteGuard<'g, *mut u8>,
}

impl<'g> InstanceWriteGuard<'g> {
    pub(crate) unsafe fn new(class: Arc<dyn Class>, data: RwLockWriteGuard<'g, *mut u8>) -> Self {
        InstanceWriteGuard { class, data }
    }

    unsafe fn cast_at<'c: 'g, U: 'static>(
        &self,
        class: &dyn Class,
        offset: usize,
    ) -> Result<&'c mut U> {
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

    pub fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe { self.cast_at(self.class.borrow(), 0) }
    }

    pub fn attr<'s>(&'s self, name: &str) -> Result<WriteReference<'g, 's>> {
        WriteReference::of(self).attr(name)
    }

    pub fn item<'s>(&'s self, index: usize) -> Result<WriteReference<'g, 's>> {
        WriteReference::of(self).item(index)
    }

    pub fn through<'s>(&'s self, lens: &View) -> Result<WriteReference<'g, 's>> {
        WriteReference::apply(lens, self)
    }
}

#[derive(Clone)]
pub struct WriteReference<'g, 'h: 'g> {
    instance: &'h InstanceWriteGuard<'g>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl<'g, 'h: 'g> WriteReference<'g, 'h> {
    pub fn of(instance: &'h InstanceWriteGuard<'g>) -> Self {
        WriteReference {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn apply(lens: &View, instance: &'h InstanceWriteGuard<'g>) -> Result<Self> {
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

    pub fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe { self.instance.cast_at(self.class.borrow(), self.offset) }
    }
}

unsafe impl<'g, 'h: 'g> IntoAccessor<WriteReference<'g, 'h>> for WriteReference<'g, 'h> {
    fn attr(self, name: &str) -> Result<WriteReference<'g, 'h>> {
        Accessor::<Lens>::attr(&*self.class, name).map(|lens| unsafe { self.access(lens) })
    }

    fn item(self, index: usize) -> Result<WriteReference<'g, 'h>> {
        Accessor::<Lens>::item(&*self.class, index).map(|lens| unsafe { self.access(lens) })
    }
}

unsafe impl<'g, 'h: 'g> IntoAccessor<WriteReference<'g, 'h>> for Result<WriteReference<'g, 'h>> {
    fn attr(self, name: &str) -> Result<WriteReference<'g, 'h>> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Result<WriteReference<'g, 'h>> {
        self.and_then(|view| view.item(index))
    }
}
