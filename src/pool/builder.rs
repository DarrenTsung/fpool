use super::{BoxedConstructor, Pool};

use std::result::Result;

pub trait Build<TPool>
where
    TPool: Pool,
{
    fn build(self) -> Result<TPool, TPool::ConstructionError>;
}

pub struct Builder<TPool>
where
    TPool: Pool,
    Builder<TPool>: Build<TPool>,
{
    pub(crate) pool_size: usize,
    pub(crate) constructor: BoxedConstructor<TPool::Item, TPool::ConstructionError>,
}

impl<TPool> Builder<TPool>
where
    TPool: Pool,
    Builder<TPool>: Build<TPool>,
{
    pub fn new<F>(pool_size: usize, constructor: F) -> Builder<TPool>
    where
        F: 'static + Send + Fn() -> Result<TPool::Item, TPool::ConstructionError>,
    {
        let constructor = Box::new(constructor);

        Builder {
            pool_size,
            constructor,
        }
    }

    pub fn build(self) -> Result<TPool, TPool::ConstructionError> {
        Build::<TPool>::build(self)
    }
}
