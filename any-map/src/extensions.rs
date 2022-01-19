use crate::{AnyMap, AnyMapSync};
#[cfg(feature = "cloneable-any")]
use crate::{CloneableAnyMap, CloneableAnyMapSync};

macro_rules! define_extensions {
    ($(#[$attr:meta])* $name:ident, $any_map_struct:tt, $any_or_cloneable_any_trait:tt $(+ $send_sync_trait_and_others:tt)*, $static_lifetime:tt $(+ $clone_trait_and_others:tt)*) => {
        $(#[$attr])*
        #[derive(Default, Debug)]
        pub struct $name(Option<Box<$any_map_struct>>);

        // Ref https://github.com/hyperium/http/blob/v0.2.6/src/extensions.rs#L48-L190
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self(Some(Box::new($any_map_struct::with_capacity(capacity))))
            }

            pub fn insert<T: $static_lifetime $(+ $clone_trait_and_others)* $(+ $send_sync_trait_and_others)*>(&mut self, val: T) -> Option<T> {
                self.0
                    .get_or_insert_with(|| Box::new($any_map_struct::default())).insert(val)
            }

            pub fn get<T: $static_lifetime $(+ $clone_trait_and_others)*>(&self) -> Option<&T> {
                self.0
                    .as_ref()
                    .and_then(|x| x.get::<T>())
            }

            pub fn get_mut<T: $static_lifetime $(+ $clone_trait_and_others)*>(&mut self) -> Option<&mut T> {
                self.0
                    .as_mut()
                    .and_then(|x| x.get_mut::<T>())
            }

            pub fn remove<T: $static_lifetime $(+ $clone_trait_and_others)*>(&mut self) -> Option<T> {
                self.0
                    .as_mut()
                    .and_then(|x| x.remove::<T>())
            }

            pub fn contains<T: $static_lifetime $(+ $clone_trait_and_others)*>(&self) -> bool {
                self.0
                    .as_ref()
                    .and_then(|x| Some(x.contains::<T>())) == Some(true)
            }
        }

        impl $name {
            pub fn get_inner(&self) -> Option<&Box<$any_map_struct>> {
                self.0
                    .as_ref()
            }
        }
    }
}

//
define_extensions!(Extensions, AnyMap, Any, 'static);

define_extensions!(
    ExtensionsSync,
    AnyMapSync, Any + Send + Sync, 'static
);

#[cfg(feature = "cloneable-any")]
define_extensions!(
    #[derive(Clone)]
    CloneableExtensions,
    CloneableAnyMap,
    CloneableAny,
    'static + Clone
);

#[cfg(feature = "cloneable-any")]
define_extensions!(
    #[derive(Clone)]
    CloneableExtensionsSync,
    CloneableAnyMapSync,
    CloneableAny + Send + Sync,
    'static + Clone
);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo(usize);

    #[derive(Clone)]
    struct Bar(usize);

    #[test]
    fn test_extensions() {
        let mut extensions = Extensions::default();

        assert!(!extensions.contains::<Foo>());

        assert!(extensions.insert(Foo(1)).is_none());

        assert!(extensions.contains::<Foo>());
        assert_eq!(extensions.get_inner().map(|x| x.len()), Some(1));

        assert_eq!(extensions.get::<Foo>().unwrap(), &Foo(1));
        extensions.get_mut::<Foo>().map(|x| {
            x.0 = 2;
            x
        });
        assert_eq!(extensions.get::<Foo>().unwrap(), &Foo(2));
        assert!(extensions.remove::<Foo>().is_some());

        assert!(!extensions.contains::<Foo>());
        assert_eq!(extensions.get_inner().map(|x| x.len()), Some(0));

        println!("{:?}", extensions);
    }

    #[test]
    fn test_extensions_sync() {
        let mut extensions = ExtensionsSync::new();

        assert!(extensions.insert(Foo(1)).is_none());

        assert!(extensions.contains::<Foo>());
        assert_eq!(extensions.get_inner().map(|x| x.len()), Some(1));

        println!("{:?}", extensions);
    }

    #[cfg(feature = "cloneable-any")]
    #[test]
    fn test_cloneable_extensions() {
        #[derive(Debug, Clone)]
        struct Wrapper(CloneableExtensions);

        let mut extensions = CloneableExtensions::with_capacity(1);

        assert!(extensions.insert(Bar(1)).is_none());

        assert!(extensions.contains::<Bar>());
        assert_eq!(extensions.get_inner().map(|x| x.len()), Some(1));

        println!("{:?}", extensions);

        let wrapper = Wrapper(extensions);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();
    }

    #[cfg(feature = "cloneable-any")]
    #[test]
    fn test_cloneable_extensions_sync() {
        #[derive(Debug, Clone)]
        struct Wrapper(CloneableExtensionsSync);

        let mut extensions = CloneableExtensionsSync::new();

        assert!(extensions.insert(Bar(1)).is_none());

        assert!(extensions.contains::<Bar>());
        assert_eq!(extensions.get_inner().map(|x| x.len()), Some(1));

        println!("{:?}", extensions);

        let wrapper = Wrapper(extensions);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();
    }
}
