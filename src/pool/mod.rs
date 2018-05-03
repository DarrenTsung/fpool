#[macro_use] mod macros;
mod builder;
mod error;
mod round_robin_pool;

pub use self::round_robin_pool::RoundRobinPool;
pub use self::error::{
    Error, Result,
    ConstructionError, ConstructionResult,
};

pub type BoxedConstructor<T> = Box<Fn() -> ConstructionResult<T>>;

pub trait Pool<T> {
    /// Acts on the next item of the pool
    /// Return false if the item is invalid and needs to be recreated
    fn act<F>(&mut self, action: F) -> Result<()>
    where
        F: Fn(&T) -> bool;

    /// Acts on the next item of the pool, mutably
    /// Return false if the item is invalid and needs to be recreated
    fn act_mut<F>(&mut self, action: F) -> Result<()>
    where
        F: Fn(&mut T) -> bool;
}
