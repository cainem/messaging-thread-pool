use crate::{pool_item::PoolItem, ThreadPool};

impl<P> Drop for ThreadPool<P>
where
    P: PoolItem,
{
    /// implement drop to shutdown all of the thread pools threads
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use crate::{samples::*, ThreadPool};

    #[test]
    fn one_thread_drop_clean_shutdown_as_expected() {
        let target = ThreadPool::<Randoms>::new(1);

        drop(target);
    }
}
