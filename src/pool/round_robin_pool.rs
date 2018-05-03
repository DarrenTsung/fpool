use super::{BoxedConstructor, Result, ConstructionResult, Pool};
use super::builder::{Builder, Build};

pub struct RoundRobinPool<T> {
    items: Vec<T>,
    constructor: BoxedConstructor<T>,
    index: usize,
}

impl<T> RoundRobinPool<T> {
    builder_fn!(RoundRobinPool);
    pool_fns!();

    fn get_next(&mut self) -> &mut T {
        let old_index = self.index;
        self.index = (self.index + 1) % self.items.len();
        self.items.get_mut(old_index).expect("exists")
    }

    fn reconstruct_last_item(&mut self) -> ConstructionResult<()> {
        let previous_index =
        {
            if self.index == 0 {
                self.items.len() - 1
            } else {
                self.index - 1
            }
        };

        self.items[previous_index] = (self.constructor)()?;
        Ok(())
    }
}

impl<T> Pool<T> for RoundRobinPool<T> {
    fn act<F>(&mut self, action: F)
    -> Result<()>
    where
        F: Fn(&T) -> bool
    {
        if !action(self.get_next()) {
            self.reconstruct_last_item()?;
        }
        Ok(())
    }

    fn act_mut<F>(&mut self, action: F)
    -> Result<()>
    where
        F: Fn(&mut T) -> bool
    {
        if !action(self.get_next()) {
            self.reconstruct_last_item()?;
        }
        Ok(())
    }
}

impl<T> Build<RoundRobinPool<T>> for Builder<T, RoundRobinPool<T>> {
    fn build(self) -> Result<RoundRobinPool<T>> {
        let Builder { pool_size, constructor, .. } = self;

        let mut items = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(constructor()?);
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

        let mut pool = RoundRobinPool::builder(5, constructor)
            .build()
            .expect("constructors successful");

        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 2); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 3); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 2); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 3); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
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

        let mut pool = RoundRobinPool::builder(5, constructor)
            .build()
            .expect("constructors successful");

        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); true }).unwrap();

        // 2 will be removed and new constructor will replace with 5
        pool.act(|item| { assert_eq!(*item, 2); false }).unwrap();
        // 3 will be removed and new constructor will replace with 6
        pool.act(|item| { assert_eq!(*item, 3); false }).unwrap();

        pool.act(|item| { assert_eq!(*item, 4); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 5); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 6); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); true }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); true }).unwrap();
    }
}
