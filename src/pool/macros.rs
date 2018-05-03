macro_rules! builder_fn {
    ($pool_type:ident) => {
        pub fn builder<F: 'static + Fn() -> ConstructionResult<T>>(
            pool_size: usize,
            constructor: F,
        ) -> Builder<T, $pool_type<T>>
        {
            Builder::new(pool_size, constructor)
        }
    }
}

macro_rules! pool_fns {
    () => {
        pub fn act<F>(&mut self, action: F) -> Result<()>
        where
            F: Fn(&T) -> bool
        {
            Pool::act(self, action)
        }

        pub fn act_mut<F>(&mut self, action: F) -> Result<()>
        where
            F: Fn(&mut T) -> bool
        {
            Pool::act_mut(self, action)
        }
    }
}
