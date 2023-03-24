use crate::class::Class;
use crate::error::{Error, Result};
use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Member {
    pub name: String,
    pub class: Arc<dyn Class>,
    pub offset: usize,
}

#[derive(Clone)]
pub struct ObjectClass {
    pub name: String,
    pub base: Option<Arc<dyn Class>>,
    members: Vec<Member>,
    lookup: HashMap<String, usize>, // TODO: share with Member
    pub size: usize,
}

impl ObjectClass {
    pub fn new(builder: Builder) -> Self {
        ObjectClass {
            name: builder.name,
            base: builder.base,
            members: builder.members,
            lookup: builder.lookup,
            size: builder.size,
        }
    }
}

unsafe impl Class for ObjectClass {
    fn size(&self) -> usize {
        self.size
    }

    fn align(&self) -> usize {
        if self.members.len() == 0 {
            1
        } else {
            self.members
                .iter()
                .map(|member| member.class.align())
                .max()
                .unwrap_or(1)
        }
    }

    fn layout(&self) -> Layout {
        // Needs to be a power of two
        // TODO: use std::ptr::Alignment when stable
        Layout::from_size_align(self.size, self.align()).unwrap()
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }

    unsafe fn construct(&self, data: *mut u8) {
        for member in self.members.iter() {
            unsafe {
                let address = data.add(member.offset);
                member.class.construct(address);
            }
        }
    }

    unsafe fn destroy(&self, data: *mut u8) {
        for member in self.members.iter().rev() {
            unsafe {
                let address = data.add(member.offset);
                member.class.destroy(address);
            }
        }
    }

    fn attr(&self, name: &str) -> Result<(Arc<dyn Class>, usize)> {
        if let Some(member_index) = self.lookup.get(name) {
            // Constructed so every key is always a valid index, immutable.
            unsafe {
                let member = self.members.get_unchecked(*member_index);
                Ok((member.class.clone(), member.offset))
            }
        } else {
            Err(Error::AttributeError(format!(
                "Class {} has no attribute {}", self.name(), name
            )))
        }
    }
}

#[derive(Clone)]
pub struct Builder {
    pub name: String,
    pub base: Option<Arc<dyn Class>>,
    members: Vec<Member>,
    lookup: HashMap<String, usize>,
    pub size: usize,
}

fn align(offset: usize, align: usize) -> usize {
    if align == 0 {
        offset
    } else {
        (offset + align - 1) / align * align
    }
}

impl Builder {
    pub fn new(name: String) -> Self {
        Builder {
            name,
            base: None,
            members: Vec::new(),
            lookup: HashMap::new(),
            size: 0,
        }
    }

    pub fn new_inherit(name: String, base: Arc<ObjectClass>) -> Self {
        Builder {
            name,
            members: base.members.clone(),
            lookup: base.lookup.clone(),
            size: base.size,
            base: Some(base),
        }
    }

    pub fn add(&mut self, name: String, class: Arc<dyn Class>) {
        let offset = align(self.size, class.align());
        self.size = offset + class.size();
        self.lookup.insert(name.clone(), self.members.len());
        self.members.push(Member {
            name,
            class,
            offset,
        });
    }
}
