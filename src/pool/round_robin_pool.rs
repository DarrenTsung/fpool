use super::{ItemHandle, BoxedConstructor, Pool};
use super::builder::{Builder, Build};

/// A [`Pool`] that uses round-robin logic to retrieve items
/// in the pool. Can be converted into the items with the `into_items()` function.
pub struct RoundRobinPool<T, TCError> {
    items: Vec<ItemHandle<T>>,
    constructor: BoxedConstructor<T, TCError>,
    index: usize,
}

impl<T, TCError> RoundRobinPool<T, TCError> {
    pub_builder_fn!(RoundRobinPool);
    pub_pool_fns!();

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn into_items(self) -> impl Iterator<Item=T> {
        self.items.into_iter().map(|h| h.into_item())
    }
}

impl<T, TCError> Pool for RoundRobinPool<T, TCError> {
    type Item = T;
    type ConstructionError = TCError;

    fn get(&mut self) -> Result<&mut ItemHandle<T>, TCError>
    {
        let invalid = self.items[self.index].invalid();
        if invalid {
            self.items[self.index] = ItemHandle::new((self.constructor)()?);
        }

        let old_index = self.index;
        self.index = (self.index + 1) % self.items.len();
        Ok(&mut self.items[old_index])
    }
}

impl<T, TCError>
    Build<RoundRobinPool<T, TCError>> for Builder<RoundRobinPool<T, TCError>>
{
    fn build(self) -> Result<RoundRobinPool<T, TCError>, TCError> {
        let Builder { pool_size, constructor, .. } = self;

        let mut items = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(ItemHandle::new(constructor()?));
        }

        Ok(RoundRobinPool {
            items,
            constructor,
            index: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_in_round_robin_fashion() {
        static mut INDEX : usize = 0;

        let constructor = || {
            unsafe {
                let old_index = INDEX;
                INDEX += 1;
                Ok(old_index)
            }
        };

        let mut pool = RoundRobinPool::<_, ()>::builder(5, constructor)
            .build()
            .expect("constructors successful");

        assert_eq!(*pool.get().unwrap().as_item(), 0);
        assert_eq!(*pool.get().unwrap().as_item(), 1);
        assert_eq!(*pool.get().unwrap().as_item(), 2);
        assert_eq!(*pool.get().unwrap().as_item(), 3);
        assert_eq!(*pool.get().unwrap().as_item(), 4);
        assert_eq!(*pool.get().unwrap().as_item(), 0);
        assert_eq!(*pool.get().unwrap().as_item(), 1);
        assert_eq!(*pool.get().unwrap().as_item(), 2);
        assert_eq!(*pool.get().unwrap().as_item(), 3);
        assert_eq!(*pool.get().unwrap().as_item(), 4);
        assert_eq!(*pool.get().unwrap().as_item(), 0);
    }

    #[test]
    fn refills_items_as_expected() {
        static mut INDEX : usize = 0;

        let constructor = || {
            unsafe {
                let old_index = INDEX;
                INDEX += 1;
                Ok(old_index)
            }
        };

        let mut pool = RoundRobinPool::<_, ()>::builder(5, constructor)
            .build()
            .expect("constructors successful");

        assert_eq!(*pool.get().unwrap().as_item(), 0);
        assert_eq!(*pool.get().unwrap().as_item(), 1);

        // 2 will be removed and new constructor will replace with 5
        {
            let item_2 = pool.get().unwrap();
            assert_eq!(*item_2.as_item(), 2);
            item_2.invalidate();
        }

        // 3 will be removed and new constructor will replace with 6
        {
            let item_3 = pool.get().unwrap();
            assert_eq!(*item_3.as_item(), 3);
            item_3.invalidate();
        }

        assert_eq!(*pool.get().unwrap().as_item(), 4);
        assert_eq!(*pool.get().unwrap().as_item(), 0);
        assert_eq!(*pool.get().unwrap().as_item(), 1);
        assert_eq!(*pool.get().unwrap().as_item(), 5);
        assert_eq!(*pool.get().unwrap().as_item(), 6);
        assert_eq!(*pool.get().unwrap().as_item(), 4);
        assert_eq!(*pool.get().unwrap().as_item(), 0);
    }
}
