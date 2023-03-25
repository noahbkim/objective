pub mod accessor;
pub mod class;
pub mod error;
pub mod instance;

#[cfg(test)]
mod tests {
    use crate::accessor::{Accessor, Cast, IntoAccessor, MutableCast};
    use crate::class::array::Array;
    use crate::class::object::{Builder, Object};
    use crate::class::value::Value;
    use crate::class::Class;
    use crate::instance::Instance;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let class: Arc<dyn Class> = Arc::new(Value::<u64>::new());
        let instance = Instance::new(class);

        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 0);
        *instance.write().unwrap().cast::<u64>().unwrap() = 69;
        assert_eq!(*instance.read().unwrap().cast::<u64>().unwrap(), 69);
    }

    #[test]
    fn object_creation() {
        let u64_class: Arc<dyn Class> = Arc::new(Value::<u64>::new());
        let i32_class: Arc<dyn Class> = Arc::new(Value::<i32>::new());

        let mut builder = Builder::new("Foo".into());
        builder.add("a".into(), u64_class.clone());
        builder.add("b".into(), i32_class.clone());
        builder.add("c".into(), i32_class.clone());
        assert_eq!(builder.size, 16);

        let foo_class = Arc::new(Object::new(builder));
        let foo = Arc::new(Instance::new(foo_class.clone()));

        assert_eq!(
            *foo.read()
                .unwrap()
                .attr("a")
                .unwrap()
                .cast::<u64>()
                .unwrap(),
            0
        );
        *foo.write()
            .unwrap()
            .attr("a")
            .unwrap()
            .cast::<u64>()
            .unwrap() = 69;
        assert_eq!(
            *foo.read()
                .unwrap()
                .attr("a")
                .unwrap()
                .cast::<u64>()
                .unwrap(),
            69
        );

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
        assert_eq!(
            *foo.read()
                .unwrap()
                .through(&c_lens)
                .unwrap()
                .cast::<i32>()
                .unwrap(),
            -420
        );

        let foo_array_class: Arc<dyn Class> = Arc::new(Array::new(foo_class.clone(), 3));
        let foo_array = Arc::new(Instance::new(foo_array_class));

        assert_eq!(
            *foo_array
                .read()
                .unwrap()
                .item(0)
                .attr("a")
                .unwrap()
                .cast::<u64>()
                .unwrap(),
            0
        );
        *foo_array
            .write()
            .unwrap()
            .item(2)
            .attr("b")
            .unwrap()
            .cast::<i32>()
            .unwrap() = 300;
        *foo_array
            .write()
            .unwrap()
            .item(1)
            .attr("b")
            .unwrap()
            .cast::<i32>()
            .unwrap() = 200;
        *foo_array
            .write()
            .unwrap()
            .item(0)
            .attr("b")
            .unwrap()
            .cast::<i32>()
            .unwrap() = 100;
        assert_eq!(
            *foo_array
                .read()
                .unwrap()
                .item(2)
                .attr("b")
                .unwrap()
                .cast::<i32>()
                .unwrap(),
            300
        );
    }
}
