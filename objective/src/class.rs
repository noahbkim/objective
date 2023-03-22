pub mod array;
pub mod lens;
pub mod object;
pub mod value;

use std::alloc::Layout;
use std::any::TypeId;
use std::sync::Arc;

// Unsafe: this trait is fucked up bruh
//   - construct() expects a buffer with length >= size()
//   - construct() guarantees that data may be cast as the corresponding T of id()
//   - destroy() expects that data has been constructed() by this type
pub unsafe trait Class {
    fn size(&self) -> usize;
    fn align(&self) -> usize;
    fn layout(&self) -> Layout;
    fn name(&self) -> &str;

    unsafe fn id(&self) -> Option<TypeId> { None }
    unsafe fn construct(&self, data: *mut u8);
    unsafe fn destroy(&self, data: *mut u8);

    fn attr(&self, _name: &str) -> Option<(Arc<dyn Class>, usize)> {
        None
    }
    fn item(&self, _index: usize) -> Option<(Arc<dyn Class>, usize)> {
        None
    }
}
