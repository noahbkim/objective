use std::mem::{size_of, align_of};
use std::any::TypeId;
use std::marker::PhantomData;
use std::rc::Rc;

// Unsafe: this trait is fucked up bruh
//   - construct() expects a buffer with length >= size()
//   - construct() guarantees that data may be cast as the corresponding T of id()
//   - destroy() expects that data has been constructed() by this type
unsafe trait Type {
    fn size(&self) -> usize;
    fn align(&self) -> usize;
    fn id(&self) -> Option<TypeId>;
    unsafe fn construct(&self, data: *mut u8);
    unsafe fn destroy(&self, data: *mut u8);
}

struct ValueType<T> {
    phantom_data: PhantomData<T>,
}

impl<T: 'static> ValueType<T> {
    fn new() -> Self {
        return ValueType {
            phantom_data: PhantomData::default(),
        }
    }
}

unsafe impl<T> Type for ValueType<T>
    where T: Sized + Default + 'static
{
    fn size(&self) -> usize {
        size_of::<T>()
    }

    fn align(&self) -> usize {
        align_of::<T>()
    }

    fn id(&self) -> Option<TypeId> { Some(TypeId::of::<T>()) }

    unsafe fn construct(&self, data: *mut u8) {
        data.cast::<T>().write(T::default());
    }

    unsafe fn destroy(&self, data: *mut u8) {
        data.cast::<T>().drop_in_place();
    }
}

#[derive(Debug)]
struct InvalidCast {}

struct Instance {
    class: Rc<dyn Type>,
    data: Box<[u8]>,
}

impl Instance {
    pub fn new(class: Rc<dyn Type>) -> Self {
        let mut data = Vec::new();

        // Invariant: construct expects to have at least size() data
        data.resize(class.size(), 0);
        unsafe {
            class.construct(data.as_mut_ptr());
        }

        Self { class, data: data.into_boxed_slice() }
    }

    pub fn borrow<U: 'static>(&self) -> Result<&U, InvalidCast> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                Ok(&*self.data.as_ptr().cast::<U>())
            }
        } else {
            Err(Invalid {})
        }
    }

    pub fn borrow_mut<U: 'static>(&mut self) -> Result<&mut U, InvalidCast> {
        // Invariant: construct() must have been called before now
        if self.class.id() == Some(TypeId::of::<U>()) {
            unsafe {
                Ok(&mut *self.data.as_mut_ptr().cast::<U>())
            }
        } else {
            Err(Invalid {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let class = Rc::new(ValueType::<u64>::new());
        let mut instance = Instance::new(class);
        assert_eq!(*instance.borrow::<u64>().unwrap(), 0);
        *instance.borrow_mut::<u64>().unwrap() = 69;
        assert_eq!(*instance.borrow::<u64>().unwrap(), 69);
    }
}
