pub enum PoolCreationError {
    ZeroSizedPool,
}

impl PoolCreationError {
    fn description(&self) -> &str {
        match self {
            PoolCreationError::ZeroSizedPool => "The pool size cannot be zero",
        }
    }
}

pub struct ThreadPool {}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // pub fn new(size: usize) -> ThreadPool {
    //     assert!(size > 0);
    //     ThreadPool {}
    // }
    pub fn buld(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError::ZeroSizedPool),
            _ => Ok(ThreadPool {}),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
