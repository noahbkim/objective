use crate::accessor::Accessor;
use crate::class::id::Id;
use crate::class::lens::Lens;
use crate::class::{Class, Metaclass, Unique};
use crate::error::{Error, Result};
use std::alloc::Layout;
use std::any::{type_name, TypeId};
use std::marker::PhantomData;
use std::mem::{align_of, size_of};

#[derive(Eq, PartialEq)]
pub struct Value<T> {
    id: Id,
    phantom_data: PhantomData<T>,
}

impl<T: 'static> Value<T> {
    pub fn new() -> Self {
        return Value {
            id: Id::new(),
            phantom_data: Default::default(),
        };
    }
}

impl<T> Unique for Value<T> {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl<T> std::fmt::Debug for Value<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", type_name::<T>())
    }
}

unsafe impl<T> Accessor<Lens> for Value<T> {
    fn attr(&self, _: &str) -> Result<Lens> {
        Err(Error::TypeError(format!(
            "Value class {:?} does not support attribute access!",
            self
        )))
    }

    fn item(&self, _: usize) -> Result<Lens> {
        Err(Error::TypeError(format!(
            "Value class {:?} does not support index access!",
            self
        )))
    }
}

unsafe impl<T: Default> Metaclass for Value<T> {
    unsafe fn construct(&self, data: *mut u8) {
        data.cast::<T>().write(T::default());
    }

    unsafe fn destroy(&self, data: *mut u8) {
        data.cast::<T>().drop_in_place();
    }
}

unsafe impl<T> Class for Value<T>
where
    T: Sized + Default + 'static,
{
    fn size(&self) -> usize {
        size_of::<T>()
    }

    fn align(&self) -> usize {
        align_of::<T>()
    }

    fn layout(&self) -> Layout {
        Layout::new::<T>()
    }

    fn value(&self) -> Option<TypeId> {
        Some(TypeId::of::<T>())
    }
}
