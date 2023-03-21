pub mod array;
pub mod object;
pub mod value;

use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};
use std::any::TypeId;
use std::marker::PhantomData;
use std::rc::Rc;

// Unsafe: this trait is fucked up bruh
//   - construct() expects a buffer with length >= size()
//   - construct() guarantees that data may be cast as the corresponding T of id()
//   - destroy() expects that data has been constructed() by this type
unsafe trait Class {
    fn size(&self) -> usize;
    fn align(&self) -> usize;
    fn layout(&self) -> Layout;
    fn id(&self) -> Option<TypeId>;
    unsafe fn construct(&self, data: *mut u8);
    unsafe fn destroy(&self, data: *mut u8);
}

struct ValueType<T> {
    phantom_data: PhantomData<T>,
}

impl<T: 'static> ValueType<T> {
    fn new() -> Self {
        return ValueType { phantom_data: Default::default() }
    }
}

unsafe impl<T> Class for ValueType<T>
    where T: Sized + Default + 'static
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
