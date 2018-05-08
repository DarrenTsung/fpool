use super::{ActResult, BoxedConstructor, Pool};
use super::builder::{Builder, Build};

pub struct RoundRobinPool<T, TError> {
    items: Vec<T>,
    constructor: BoxedConstructor<T, TError>,
    index: usize,
}

impl<T, TError> RoundRobinPool<T, TError> {
    pub_builder_fn!(RoundRobinPool);
    pub_pool_fns!();

    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    fn get_next(&mut self) -> &mut T {
        let old_index = self.index;
        self.index = (self.index + 1) % self.items.len();
        self.items.get_mut(old_index).expect("exists")
    }

    fn reconstruct_last_item(&mut self) -> Result<(), TError> {
        let previous_index = {
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

impl<T, TError> Pool for RoundRobinPool<T, TError> {
    type Item = T;
    type Error = TError;

    fn act<F>(&mut self, action: F)
    -> Result<(), Self::Error>
    where
        F: FnOnce(&mut Self::Item) -> ActResult<TError>
    {
        let mut res = Ok(());

        match action(self.get_next()) {
            ActResult::Valid => (),
            ActResult::ValidWithError(err) => {
                res = Err(err);
            },
            ActResult::Invalid => {
                self.reconstruct_last_item()?;
            }
            ActResult::InvalidWithError(err) => {
                res = Err(err);
                self.reconstruct_last_item()?;
            },
        }

        res
    }
}

impl<T, TError> Build<RoundRobinPool<T, TError>> for Builder<RoundRobinPool<T, TError>> {
    fn build(self) -> Result<RoundRobinPool<T, TError>, TError> {
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

        let mut pool = RoundRobinPool::<_, ()>::builder(5, constructor)
            .build()
            .expect("constructors successful");

        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 2); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 3); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 2); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 3); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
    }

    #[test]
    fn refills_items_as_expected() {
        static mut INDEX : usize = 0;

        let constructor = || {
            unsafe {
                let old_index = INDEX;
                INDEX += 1;
                let ret: Result<_, ()> = Ok(old_index);
                ret
            }
        };

        let mut pool = RoundRobinPool::builder(5, constructor)
            .build()
            .expect("constructors successful");

        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); ActResult::Valid }).unwrap();

        // 2 will be removed and new constructor will replace with 5
        pool.act(|item| { assert_eq!(*item, 2); ActResult::Invalid }).unwrap();
        // 3 will be removed and new constructor will replace with 6
        pool.act(|item| { assert_eq!(*item, 3); ActResult::Invalid }).unwrap();

        pool.act(|item| { assert_eq!(*item, 4); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 1); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 5); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 6); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 4); ActResult::Valid }).unwrap();
        pool.act(|item| { assert_eq!(*item, 0); ActResult::Valid }).unwrap();
    }
}
