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

impl fmt::Debug for (dyn CloneableAny) {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CloneableAny")
            .field(&format_args!("_"))
            .finish()
    }
}
impl fmt::Debug for (dyn CloneableAny + Send) {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CloneableAny")
            .field(&format_args!("_"))
            .finish()
    }
}
impl fmt::Debug for (dyn CloneableAny + Send + Sync) {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CloneableAny")
            .field(&format_args!("_"))
            .finish()
    }
}

impl<T> CloneableAny for T where T: 'static + Clone {}

//
//
//
pub trait CloneableAnySync: DowncastSync + DynClone {}

impl_downcast!(CloneableAnySync);
clone_trait_object!(CloneableAnySync);

impl fmt::Debug for (dyn CloneableAnySync) {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CloneableAnySync")
            .field(&format_args!("_"))
            .finish()
    }
}

impl<T> CloneableAnySync for T where T: 'static + Clone + Send + Sync {}
