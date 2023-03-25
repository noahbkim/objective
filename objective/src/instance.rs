pub mod read;
pub mod write;

use crate::class::Class;
use crate::instance::read::InstanceReadGuard;
use crate::instance::write::InstanceWriteGuard;
use std::alloc::{alloc, dealloc};
use std::sync::{Arc, PoisonError, RwLock};

pub struct Instance {
    class: Arc<dyn Class>,
    data: RwLock<*mut u8>,
}

impl Instance {
    pub fn new(class: Arc<dyn Class>) -> Self {
        // Invariant: construct expects to have at least size() data
        // Must be deallocated in drop
        unsafe {
            let data = alloc(class.layout());
            class.construct(data);
            Self {
                class,
                data: RwLock::new(data),
            }
        }
    }

    pub fn read(&self) -> Result<InstanceReadGuard, PoisonError<InstanceReadGuard>> {
        InstanceReadGuard::acquire(self)
    }

    pub fn write(&self) -> Result<InstanceWriteGuard, PoisonError<InstanceWriteGuard>> {
        InstanceWriteGuard::acquire(self)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // Invariant: is not null, has layout of self.class.layout()
        unsafe {
            dealloc(*self.data.write().unwrap(), self.class.layout());
        }
    }
}
