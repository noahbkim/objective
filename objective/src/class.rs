pub mod array;
pub mod id;
pub mod lens;
pub mod object;
pub mod value;
pub mod view;

use crate::accessor::Accessor;
use crate::class::id::Id;
use crate::class::lens::Lens;
use std::alloc::Layout;
use std::any::TypeId;

pub trait Unique {
    fn id(&self) -> &Id;
}

pub unsafe trait Metaclass {
    unsafe fn construct(&self, data: *mut u8);
    unsafe fn destroy(&self, data: *mut u8);
}

// Unsafe: this trait is fucked up bruh
//   - construct() expects a buffer with length >= size()
//   - construct() guarantees that data may be cast as the corresponding T of id()
//   - destroy() expects that data has been constructed() by this type
pub unsafe trait Class: Metaclass + Accessor<Lens> + Unique + std::fmt::Debug {
    fn size(&self) -> usize;
    fn align(&self) -> usize;
    fn layout(&self) -> Layout;
    fn value(&self) -> Option<TypeId> {
        None
    }
}
