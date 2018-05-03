use super::{Result, Pool, BoxedConstructor, ConstructionResult};

use std::marker::PhantomData;

pub trait Build<TPool> {
    fn build(self) -> Result<TPool>;
}

pub struct Builder<T, TPool>
where
    TPool: Pool<T>,
    Builder<T, TPool>: Build<TPool>
{
    pub(crate) pool_size: usize,
    pub(crate) constructor: BoxedConstructor<T>,

    _pool: PhantomData<TPool>,
}

impl<T, TPool> Builder<T, TPool>
where
    TPool: Pool<T>,
    Builder<T, TPool>: Build<TPool>
{
    pub fn new<F> (
        pool_size: usize,
        constructor: F,
    ) -> Builder<T, TPool>
    where
        F: 'static + Fn() -> ConstructionResult<T>
    {
        let constructor = Box::new(constructor);

        Builder {
            pool_size,
            constructor,

            _pool: PhantomData,
        }
    }

    pub fn build(self) -> Result<TPool> {
        Build::<TPool>::build(self)
    }
}
