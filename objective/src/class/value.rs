use crate::class::Class;
use std::alloc::Layout;
use std::any::TypeId;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};

#[derive(Eq, PartialEq)]
pub struct ValueClass<T> {
    phantom_data: PhantomData<T>,
}

impl<T: 'static> ValueClass<T> {
    pub fn new() -> Self {
        return ValueClass {
            phantom_data: Default::default(),
        };
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

    fn id(&self) -> Option<TypeId> {
        Some(TypeId::of::<T>())
    }

    unsafe fn construct(&self, data: *mut u8) {
        data.cast::<T>().write(T::default());
    }

    unsafe fn destroy(&self, data: *mut u8) {
        data.cast::<T>().drop_in_place();
    }
}
