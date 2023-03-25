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
pub struct ValueClass<T> {
    id: Id,
    phantom_data: PhantomData<T>,
}

impl<T: 'static> ValueClass<T> {
    pub fn new() -> Self {
        return ValueClass {
            id: Id::new(),
            phantom_data: Default::default(),
        };
    }
}

impl<T> Unique for ValueClass<T> {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl<T> std::fmt::Debug for ValueClass<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", type_name::<T>())
    }
}

unsafe impl<T> Accessor<Lens> for ValueClass<T> {
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

unsafe impl<T: Default> Metaclass for ValueClass<T> {
    unsafe fn construct(&self, data: *mut u8) {
        data.cast::<T>().write(T::default());
    }

    unsafe fn destroy(&self, data: *mut u8) {
        data.cast::<T>().drop_in_place();
    }
}

unsafe impl<T> Class for ValueClass<T>
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
