pub mod class;
pub mod instance;
pub mod error;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::class::Class;
    use crate::class::object::{ObjectClass, Builder};
    use crate::class::value::ValueClass;
    use crate::instance::Instance;
    use crate::instance::view::Viewable;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let class: Arc<dyn Class> = Arc::new(ValueClass::<u64>::new());
        let instance = Instance::new(class);
        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 0);
        *instance.write().unwrap().cast::<u64>().unwrap() = 69;
        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 69);
    }

    #[test]
    fn object_creation() -> Result<(), Box<dyn Error>> {
        let u64_class: Arc<dyn Class> = Arc::new(ValueClass::<u64>::new());
        let i32_class: Arc<dyn Class> = Arc::new(ValueClass::<i32>::new());

        let mut builder = Builder::new("Foo".into());
        builder.add("a".into(), u64_class.clone());
        builder.add("b".into(), i32_class.clone());
        builder.add("c".into(), i32_class.clone());
        assert_eq!(builder.size, 16);

        let foo_class = Arc::new(ObjectClass::new(builder));
        let foo = Arc::new(Instance::new(foo_class.clone()));
        assert_eq!(*foo.clone().attr("a")?.read().unwrap().cast::<u64>()?, 0);
        *foo.clone().attr("a").unwrap().write().unwrap().cast::<u64>().unwrap() = 69;
        assert_eq!(*foo.clone().attr("a").unwrap().read().unwrap().cast::<u64>().unwrap(), 69);

        *foo.clone().attr("b").unwrap().write().unwrap().cast::<i32>().unwrap() = -69;
        *foo.clone().attr("c").unwrap().write().unwrap().cast::<i32>().unwrap() = -420;
        assert_eq!(*foo.clone().attr("a").unwrap().read().unwrap().cast::<u64>().unwrap(), 69);
        assert_eq!(*foo.clone().attr("b").unwrap().read().unwrap().cast::<i32>().unwrap(), -69);
        assert_eq!(*foo.clone().attr("c").unwrap().read().unwrap().cast::<i32>().unwrap(), -420);

        Ok(())
    }
}
