macro_rules! pub_builder_fn {
    ($pool_type:ident) => {
        pub fn builder<F: 'static + Fn() -> Result<T, TError>>(
            pool_size: usize,
            constructor: F,
        ) -> Builder<$pool_type<T, TError>>
        {
            Builder::new(pool_size, constructor)
        }
    }
}

macro_rules! pub_pool_fns {
    () => {
        pub fn act<F>(&mut self, action: F) -> Result<(), TError>
        where
            F: FnOnce(&mut T) -> ActResult<TError>
        {
            Pool::act(self, action)
        }
    }
}
