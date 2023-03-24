use std::sync::atomic::{AtomicUsize, Ordering};

static AUTOINCREMENT: AtomicUsize = AtomicUsize::new(1);

#[derive(Eq, PartialEq, Debug)]
pub struct Id {
    value: usize,
}

impl Id {
    pub fn new() -> Self {
        Id { value: AUTOINCREMENT.fetch_add(1, Ordering::Relaxed) }
    }
}
