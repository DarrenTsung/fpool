#[macro_use] mod macros;
mod builder;
mod round_robin_pool;

pub use self::round_robin_pool::RoundRobinPool;

pub type BoxedConstructor<T, TError> = Box<Fn() -> Result<T, TError>>;

pub enum ActResult<TError> {
    /// Return Valid if the item does not need to be recreated
    Valid,
    ValidWithError(TError),

    /// Return Invalid if the item needs to be recreated
    Invalid,
    InvalidWithError(TError),
}

pub trait Pool {
    type Item;
    type Error;

    /// Acts on the next item of the pool
    fn act<F>(&mut self, action: F) -> Result<(), Self::Error>
    where
        F: FnOnce(&mut Self::Item) -> ActResult<Self::Error>;
}
