use crate::error::Result;

pub unsafe trait Accessor<T> {
    fn attr(&self, name: &str) -> Result<T>;
    fn item(&self, index: usize) -> Result<T>;
}

pub unsafe trait IntoAccessor<T> {
    fn attr(self, name: &str) -> Result<T>;
    fn item(self, index: usize) -> Result<T>;
}
