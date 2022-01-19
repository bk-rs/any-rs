use core::{
    any::{Any, TypeId},
    hash::BuildHasherDefault,
    ops::Deref,
};
use std::collections::HashMap;

#[cfg(feature = "cloneable-any")]
use cloneable_any::CloneableAny;

pub mod hasher;

use self::hasher::Hasher;

//
macro_rules! define_any_map {
    ($(#[$attr:meta])* $name:ident, $any_or_cloneable_any_trait:tt $(+ $send_sync_trait_and_others:tt)*, $static_lifetime:tt $(+ $clone_trait_and_others:tt)*) => {
        $(#[$attr])*
        #[derive(Default, Debug)]
        pub struct $name(HashMap<TypeId, Box<dyn $any_or_cloneable_any_trait $(+ $send_sync_trait_and_others)*>, BuildHasherDefault<Hasher>>);

        impl Deref for $name {
            type Target = HashMap<TypeId, Box<dyn $any_or_cloneable_any_trait $(+ $send_sync_trait_and_others)*>, BuildHasherDefault<Hasher>>;

            fn deref(&self) -> &Self::Target {
                &self.0
             }
        }

        // Ref https://github.com/hyperium/http/blob/v0.2.6/src/extensions.rs#L48-L190
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self(HashMap::with_capacity_and_hasher(capacity, Default::default()))
            }

            pub fn insert<T: $static_lifetime $(+ $clone_trait_and_others)* $(+ $send_sync_trait_and_others)*>(&mut self, val: T) -> Option<T> {
                self.0
                    .insert(TypeId::of::<T>(), Box::new(val))
                    .and_then(|boxed| {
                        (boxed as Box<dyn $any_or_cloneable_any_trait>)
                            .downcast()
                            .ok()
                            .map(|boxed| *boxed)
                    })
            }

            pub fn get<T: $static_lifetime $(+ $clone_trait_and_others)*>(&self) -> Option<&T> {
                self.0
                    .get(&TypeId::of::<T>())
                    .and_then(|boxed| (&**boxed as &(dyn $any_or_cloneable_any_trait)).downcast_ref())
            }

            pub fn get_mut<T: $static_lifetime $(+ $clone_trait_and_others)*>(&mut self) -> Option<&mut T> {
                self.0
                    .get_mut(&TypeId::of::<T>())
                    .and_then(|boxed| (&mut **boxed as &mut (dyn $any_or_cloneable_any_trait)).downcast_mut())
            }

            pub fn remove<T: $static_lifetime $(+ $clone_trait_and_others)*>(&mut self) -> Option<T> {
                self.0.remove(&TypeId::of::<T>()).and_then(|boxed| {
                    (boxed as Box<dyn $any_or_cloneable_any_trait>)
                        .downcast()
                        .ok()
                        .map(|boxed| *boxed)
                })
            }

            pub fn contains<T: $static_lifetime $(+ $clone_trait_and_others)*>(&self) -> bool {
                self.0.contains_key(&TypeId::of::<T>())
            }
        }
    }
}

//
define_any_map!(AnyMap, Any, 'static);

define_any_map!(AnyMapSync, Any + Send + Sync, 'static);

#[cfg(feature = "cloneable-any")]
define_any_map!(
    #[derive(Clone)]
    CloneableAnyMap,
    CloneableAny,
    'static + Clone
);

#[cfg(feature = "cloneable-any")]
define_any_map!(
    #[derive(Clone)]
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
    fn test_any_map() {
        let mut map = AnyMap::default();

        assert!(!map.contains::<Foo>());

        assert!(map.insert(Foo(1)).is_none());

        assert!(map.contains::<Foo>());
        assert_eq!(map.len(), 1);

        assert_eq!(map.get::<Foo>().unwrap(), &Foo(1));
        map.get_mut::<Foo>().map(|x| {
            x.0 = 2;
            x
        });
        assert_eq!(map.get::<Foo>().unwrap(), &Foo(2));
        assert!(map.remove::<Foo>().is_some());

        assert!(!map.contains::<Foo>());
        assert_eq!(map.len(), 0);

        println!("{:?}", map);
    }

    #[test]
    fn test_any_map_sync() {
        let mut map = AnyMapSync::new();

        assert!(map.insert(Foo(1)).is_none());

        assert!(map.contains::<Foo>());
        assert_eq!(map.len(), 1);

        println!("{:?}", map);
    }

    #[cfg(feature = "cloneable-any")]
    #[test]
    fn test_cloneable_any_map() {
        #[derive(Debug, Clone)]
        struct Wrapper(CloneableAnyMap);

        let mut map = CloneableAnyMap::with_capacity(1);

        assert!(map.insert(Bar(1)).is_none());

        assert!(map.contains::<Bar>());
        assert_eq!(map.len(), 1);

        println!("{:?}", map);

        let wrapper = Wrapper(map);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();
    }

    #[cfg(feature = "cloneable-any")]
    #[test]
    fn test_cloneable_any_map_sync() {
        #[derive(Debug, Clone)]
        struct Wrapper(CloneableAnyMapSync);

        let mut map = CloneableAnyMapSync::new();

        assert!(map.insert(Bar(1)).is_none());

        assert!(map.contains::<Bar>());
        assert_eq!(map.len(), 1);

        println!("{:?}", map);

        let wrapper = Wrapper(map);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();
    }
}
