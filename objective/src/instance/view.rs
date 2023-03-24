use crate::class::Class;
use crate::class::view::View;
use crate::error::{Error, Result};
use crate::instance::{InstanceReadGuard, InstanceWriteGuard};
use std::borrow::Borrow;
use std::sync::{Arc};
use crate::class::lens::Lens;

#[derive(Clone)]
pub struct ReadView<'a, 'b: 'a> {
    instance: &'b InstanceReadGuard<'a>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl<'a, 'b: 'a> ReadView<'a, 'b> {
    pub fn of<'c, 'd: 'c>(instance: &'d InstanceReadGuard<'c>) -> ReadView<'c, 'd> {
        ReadView {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn apply<'c, 'd: 'c>(lens: &View, instance: &'d InstanceReadGuard<'c>) -> Result<ReadView<'c, 'd>> {
        if lens.origin.id() == instance.class.id() {
            Ok(ReadView {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            Err(Error::TypeError(format!(
                "View of type {:?} cannot be applied to instance of type {:?}",
                lens.origin,
                instance.class
            )))
        }
    }

    fn access(self, lens: Lens) -> ReadView<'a, 'b> {
        ReadView {
            instance: &self.instance,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }

    pub fn cast<U: 'static>(&self) -> Result<&U> {
        unsafe { self.instance.cast_at(self.class.borrow(), self.offset) }
    }
}

pub trait ReadViewable<'a, 'b: 'a> {
    fn attr(self, name: &str) -> Result<ReadView<'a, 'b>>;
    fn item(self, index: usize) -> Result<ReadView<'a, 'b>>;
}

impl<'a, 'b: 'a> ReadViewable<'a, 'b> for ReadView<'a, 'b> {
    fn attr(self, name: &str) -> Result<ReadView<'a, 'b>> {
        self.class.attr(name).map(|part| self.access(part))
    }

    fn item(self, index: usize) -> Result<ReadView<'a, 'b>> {
        self.class.item(index).map(|part| self.access(part))
    }
}

impl<'a, 'b: 'a> ReadViewable<'a, 'b> for Result<ReadView<'a, 'b>> {
    fn attr(self, name: &str) -> Result<ReadView<'a, 'b>> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Result<ReadView<'a, 'b>> {
        self.and_then(|view| view.item(index))
    }
}

#[derive(Clone)]
pub struct WriteView<'a, 'b: 'a> {
    instance: &'b InstanceWriteGuard<'a>,
    class: Arc<dyn Class>,
    offset: usize,
}

impl<'a, 'b: 'a> WriteView<'a, 'b> {
    pub fn of<'c, 'd: 'c>(instance: &'d InstanceWriteGuard<'c>) -> WriteView<'c, 'd> {
        WriteView {
            class: instance.class.clone(),
            instance,
            offset: 0,
        }
    }

    pub fn apply<'c, 'd: 'c>(lens: &View, instance: &'d InstanceWriteGuard<'c>) -> Result<WriteView<'c, 'd>> {
        if std::ptr::eq(lens.origin.as_ref(), instance.class.as_ref()) {
            Ok(WriteView {
                instance,
                class: lens.class.clone(),
                offset: lens.offset,
            })
        } else {
            Err(Error::TypeError(format!(
                "View of type {:?} cannot be applied to instance of type {:?}",
                lens.origin,
                instance.class
            )))
        }
    }

    fn access(self, lens: Lens) -> WriteView<'a, 'b> {
        WriteView {
            instance: &self.instance,
            class: lens.class,
            offset: self.offset + lens.offset,
        }
    }

    pub fn cast<U: 'static>(&self) -> Result<&mut U> {
        unsafe { self.instance.cast_at(self.class.borrow(), self.offset) }
    }
}

pub trait WriteViewable<'a, 'b: 'a> {
    fn attr(self, name: &str) -> Result<WriteView<'a, 'b>>;
    fn item(self, index: usize) -> Result<WriteView<'a, 'b>>;
}

impl<'a, 'b: 'a> WriteViewable<'a, 'b> for WriteView<'a, 'b> {
    fn attr(self, name: &str) -> Result<WriteView<'a, 'b>> {
        self.class.attr(name).map(|lens| self.access(lens))
    }

    fn item(self, index: usize) -> Result<WriteView<'a, 'b>> {
        self.class.item(index).map(|lens| self.access(lens))
    }
}

impl<'a, 'b: 'a> WriteViewable<'a, 'b> for Result<WriteView<'a, 'b>> {
    fn attr(self, name: &str) -> Result<WriteView<'a, 'b>> {
        self.and_then(|view| view.attr(name))
    }

    fn item(self, index: usize) -> Result<WriteView<'a, 'b>> {
        self.and_then(|view| view.item(index))
    }
}
