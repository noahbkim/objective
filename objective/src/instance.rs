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

    pub fn read(&self) -> std::result::Result<InstanceReadGuard, PoisonError<InstanceReadGuard>> {
        match self.data.read() {
            Ok(data) => Ok(unsafe { InstanceReadGuard::new(self.class.clone(), data) }),
            Err(error) => Err(unsafe {
                PoisonError::new(InstanceReadGuard::new(
                    self.class.clone(),
                    error.into_inner(),
                ))
            }),
        }
    }

    pub fn write(
        &self,
    ) -> std::result::Result<InstanceWriteGuard, PoisonError<InstanceWriteGuard>> {
        match self.data.write() {
            Ok(data) => Ok(unsafe { InstanceWriteGuard::new(self.class.clone(), data) }),
            Err(error) => Err(unsafe {
                PoisonError::new(InstanceWriteGuard::new(
                    self.class.clone(),
                    error.into_inner(),
                ))
            }),
        }
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
