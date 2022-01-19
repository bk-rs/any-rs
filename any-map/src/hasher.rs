#[cfg(feature = "fnv")]
pub(crate) use fnv::FnvHasher as Hasher;

#[cfg(not(feature = "fnv"))]
pub(crate) use self::id_hasher::IdHasher as Hasher;

#[cfg(not(feature = "fnv"))]
pub mod id_hasher {
    // Copy from https://github.com/hyperium/http/blob/v0.2.6/src/extensions.rs#L8-L28
    #[derive(Default)]
    pub struct IdHasher(u64);

    impl core::hash::Hasher for IdHasher {
        fn write(&mut self, _: &[u8]) {
            unreachable!("TypeId calls write_u64");
        }

        #[inline]
        fn write_u64(&mut self, id: u64) {
            self.0 = id;
        }

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }
    }
}
