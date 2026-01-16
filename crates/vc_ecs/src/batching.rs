#[derive(Clone, Debug)]
pub struct BatchingStrategy {
    min_size_limit: usize,
    max_size_limit: usize,
    batches_per_thread: usize,
}

impl Default for BatchingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchingStrategy {
    /// Creates a new unconstrained default batching strategy.
    #[inline]
    pub const fn new() -> Self {
        Self {
            min_size_limit: 1,
            max_size_limit: usize::MAX,
            batches_per_thread: 1,
        }
    }

    /// Declares a batching strategy with a fixed batch size.
    #[inline]
    pub const fn fixed(batch_size: usize) -> Self {
        Self {
            min_size_limit: batch_size,
            max_size_limit: batch_size,
            batches_per_thread: 1,
        }
    }

    /// Configures the minimum allowed batch size of this instance.
    #[inline]
    pub const fn min_size(mut self, batch_size: usize) -> Self {
        self.min_size_limit = batch_size;
        self
    }

    /// Configures the maximum allowed batch size of this instance.
    #[inline]
    pub const fn max_size(mut self, batch_size: usize) -> Self {
        self.max_size_limit = batch_size;
        self
    }

    /// Configures the number of batches to assign to each thread for this instance.
    #[inline]
    pub const fn batches_per_thread(mut self, batches_per_thread: usize) -> Self {
        assert!(
            self.batches_per_thread > 0,
            "The number of batches per thread must be non-zero."
        );
        self.batches_per_thread = batches_per_thread;
        self
    }

    /// Calculate the batch size according to the given thread count and max item count.
    ///
    /// The count is provided as a closure so that it can be calculated only if needed.
    ///
    /// # Panics
    ///
    /// Panics if `thread_count` is 0.
    #[inline]
    pub fn calc_batch_size(&self, max_items: impl FnOnce() -> usize, thread_count: usize) -> usize {
        assert!(
            thread_count > 0,
            "Attempted to run parallel iteration with an empty TaskPool"
        );

        if self.max_size_limit <= self.min_size_limit {
            return self.min_size_limit;
        }

        let batches = thread_count * self.batches_per_thread;
        // Round up to the nearest batch size.
        let batch_size = max_items().div_ceil(batches);
        batch_size.clamp(self.min_size_limit, self.max_size_limit)
    }
}
