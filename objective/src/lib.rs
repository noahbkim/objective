pub mod class;
pub mod instance;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::class::Class;
    use crate::class::object::{ObjectClass, Builder};
    use crate::class::value::ValueClass;
    use crate::instance::Instance;
    use std::sync::Arc;
    use crate::class::view::ViewAccessor;

    #[test]
    fn it_works() {
        let class: Arc<dyn Class> = Arc::new(ValueClass::<u64>::new());
        let instance = Instance::new(class);

        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 0);
        *instance.write().unwrap().cast::<u64>().unwrap() = 69;
        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 69);
    }

    #[test]
    fn object_creation() {
        let u64_class: Arc<dyn Class> = Arc::new(ValueClass::<u64>::new());
        let i32_class: Arc<dyn Class> = Arc::new(ValueClass::<i32>::new());

        let mut builder = Builder::new("Foo".into());
        builder.add("a".into(), u64_class.clone());
        builder.add("b".into(), i32_class.clone());
        builder.add("c".into(), i32_class.clone());
        assert_eq!(builder.size, 16);

        let foo_class = Arc::new(ObjectClass::new(builder));
        let foo = Arc::new(Instance::new(foo_class.clone()));
        assert_eq!(*foo.read().unwrap().attr("a").unwrap().cast::<u64>().unwrap(), 0);
        *foo.write().unwrap().attr("a").unwrap().cast::<u64>().unwrap() = 69;
        assert_eq!(*foo.read().unwrap().attr("a").unwrap().cast::<u64>().unwrap(), 69);

        {
            let write = foo.write().unwrap();
            *write.attr("b").unwrap().cast::<i32>().unwrap() = -69;
            *write.attr("c").unwrap().cast::<i32>().unwrap() = -420;
        }
        {
            let read = foo.read().unwrap();
            assert_eq!(*read.attr("a").unwrap().cast::<u64>().unwrap(), 69);
            assert_eq!(*read.attr("b").unwrap().cast::<i32>().unwrap(), -69);
            assert_eq!(*read.attr("c").unwrap().cast::<i32>().unwrap(), -420);
        }

        let c_lens = foo_class.attr("c").unwrap();
        assert_eq!(*foo.read().unwrap().through(&c_lens).unwrap().cast::<i32>().unwrap(), -420);
    }
}
