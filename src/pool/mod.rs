use std::ops::{Deref, DerefMut};

#[macro_use]
mod macros;
mod builder;
mod round_robin_pool;

pub use self::round_robin_pool::RoundRobinPool;

type BoxedConstructor<T, TError> = Box<dyn Send + Fn() -> Result<T, TError>>;

/// An object which returns re-used items. Pools hold on to a constructor
/// so they can recreate elements that are invalid (marked by user).
pub trait Pool {
    type Item;
    type ConstructionError;

    /// Items are re-used unless invalid, in which case they are re-constructed.
    /// If the construction fails, then the error is returned. Do not hold on
    /// to the ItemHandle reference, as it may be returned by a subsequent call on `get()`.
    fn get(&mut self) -> Result<&mut ItemHandle<Self::Item>, Self::ConstructionError>;
}

/// A handle to the item. Implements Deref and DerefMut for the item,
/// and also allows you to invalidate the item.
#[derive(Debug)]
pub struct ItemHandle<T> {
    item: T,
    invalid: bool,
}

impl<T> ItemHandle<T> {
    pub fn as_item_mut(&mut self) -> &mut T {
        &mut self.item
    }

    pub fn as_item(&self) -> &T {
        &self.item
    }

    /// Invalidate this item, it will be re-constructed on next retrieval.
    pub fn invalidate(&mut self) {
        self.invalid = true;
    }

    pub(crate) fn invalid(&self) -> bool {
        self.invalid
    }

    pub(crate) fn new(item: T) -> ItemHandle<T> {
        ItemHandle {
            item,
            invalid: false,
        }
    }

    pub(crate) fn into_item(self) -> T {
        self.item
    }
}

impl<T> Deref for ItemHandle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.item
    }
}

impl<T> DerefMut for ItemHandle<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.item
    }
}
