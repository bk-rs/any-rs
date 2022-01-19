#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use dyn_clone::{clone_trait_object, DynClone};

//
//
//
pub trait CloneableAny: Downcast + DynClone {}

impl_downcast!(CloneableAny);
clone_trait_object!(CloneableAny);

impl fmt::Debug for dyn CloneableAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloneableAny").finish_non_exhaustive()
    }
}
impl fmt::Debug for dyn CloneableAny + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloneableAny").finish_non_exhaustive()
    }
}
impl fmt::Debug for dyn CloneableAny + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloneableAny").finish_non_exhaustive()
    }
}

impl<T> CloneableAny for T where T: 'static + Clone {}

//
//
//
pub trait CloneableAnySync: DowncastSync + DynClone {}

impl_downcast!(CloneableAnySync);
clone_trait_object!(CloneableAnySync);

impl fmt::Debug for dyn CloneableAnySync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloneableAnySync").finish_non_exhaustive()
    }
}

impl<T> CloneableAnySync for T where T: 'static + Clone + Send + Sync {}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Wrapper(Box<dyn CloneableAny + Send + Sync>);
    #[derive(Debug, Clone)]
    struct WrapperSync(Box<dyn CloneableAnySync>);

    #[derive(Clone)]
    struct Foo;

    #[test]
    fn test_debug_and_clone() {
        let wrapper = Wrapper(Box::new(Foo));
        println!("{:?}", wrapper);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();

        let wrapper = WrapperSync(Box::new(Foo));
        println!("{:?}", wrapper);
        #[allow(clippy::redundant_clone)]
        let _ = wrapper.clone();
    }

    #[test]
    fn test_downcast() {
        assert!((Wrapper(Box::new(Foo)).0 as Box<dyn CloneableAny>)
            .downcast_ref::<Foo>()
            .is_some());
        assert!((Wrapper(Box::new(Foo)).0 as Box<dyn CloneableAny>).is::<Foo>());
    }
}
