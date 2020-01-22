macro_rules! pub_builder_fn {
    ($pool_type:ident) => {
        /// Get the builder for the pool.
        pub fn builder<F: 'static + Fn() -> Result<T, TCError>>(
            pool_size: usize,
            constructor: F,
        ) -> Builder<$pool_type<T, TCError>> {
            Builder::new(pool_size, constructor)
        }
    };
}

macro_rules! pub_pool_fns {
    () => {
        /// Pass-through call to the [`Pool`] trait's `get()`.
        ///
        /// Items are re-used unless invalid, in which case they are re-constructed.
        /// If the construction fails, then the error is returned. Do not hold on
        /// to the ItemHandle reference, as it may be returned by a subsequent call on `get()`.
        pub fn get(&mut self) -> Result<&mut ItemHandle<T>, TCError> {
            Pool::get(self)
        }
    }
}
