use super::{Pool, BoxedConstructor};

use std::result::Result;

pub trait Build<TPool>
where
    TPool: Pool,
{
    fn build(self) -> Result<TPool, TPool::Error>;
}

pub struct Builder<TPool>
where
    TPool: Pool,
    Builder<TPool>: Build<TPool>,
{
    pub(crate) pool_size: usize,
    pub(crate) constructor: BoxedConstructor<TPool::Item, TPool::Error>,
}

impl<TPool> Builder<TPool>
where
    TPool: Pool,
    Builder<TPool>: Build<TPool>
{
    pub fn new<F> (
        pool_size: usize,
        constructor: F,
    ) -> Builder<TPool>
    where
        F: 'static + Fn() -> Result<TPool::Item, TPool::Error>
    {
        let constructor = Box::new(constructor);

        Builder {
            pool_size,
            constructor,
        }
    }

    pub fn build(self) -> Result<TPool, TPool::Error> {
        Build::<TPool>::build(self)
    }
}
