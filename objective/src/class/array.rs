use crate::accessor::Accessor;
use crate::class::id::Id;
use crate::class::lens::Lens;
use crate::class::{Class, Metaclass, Unique};
use crate::error::{Error, Result};
use std::alloc::Layout;
use std::sync::Arc;

pub struct Array {
    id: Id,
    pub element: Arc<dyn Class>,
    pub length: usize,
    pub size: usize,
}

impl Array {
    pub fn new(element: Arc<dyn Class>, length: usize) -> Self {
        let size = element.size() * length;
        Self {
            id: Id::new(),
            element,
            length,
            size,
        }
    }
}

impl Unique for Array {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl std::fmt::Debug for Array {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}[{}]", self.element, self.length)
    }
}

unsafe impl Accessor<Lens> for Array {
    fn attr(&self, _: &str) -> Result<Lens> {
        Err(Error::TypeError(format!(
            "Array {:?} does not support attribute access!",
            self
        )))
    }

    fn item(&self, index: usize) -> Result<Lens> {
        if index < self.length {
            Ok(Lens {
                class: self.element.clone(),
                offset: self.element.size() * index,
            })
        } else {
            Err(Error::IndexError(format!(
                "Array index {} out of bounds {}",
                index, self.length
            )))
        }
    }
}

unsafe impl Metaclass for Array {
    unsafe fn construct(&self, data: *mut u8) {
        for i in 0..self.length {
            unsafe {
                let address = data.add(self.element.size() * i);
                self.element.construct(address);
            }
        }
    }

    unsafe fn destroy(&self, data: *mut u8) {
        for i in (0..self.length).rev() {
            unsafe {
                let address = data.add(self.element.size() * i);
                self.element.destroy(address);
            }
        }
    }
}

unsafe impl Class for Array {
    fn size(&self) -> usize {
        self.size
    }

    fn align(&self) -> usize {
        self.element.align()
    }

    fn layout(&self) -> Layout {
        self.element.layout()
    }
}


