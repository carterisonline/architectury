use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;
#[cfg(feature = "green-threads")]
pub use rayon::*;

static mut POOL: Lazy<ThreadPool> = Lazy::new(|| ThreadPoolBuilder::new().build().unwrap());
static ACTIVE_THREADS: AtomicU64 = AtomicU64::new(0);

/// Spawns an asynchronous green thread. Under the hood, green threads are spawned
/// in a global work-stealing thread pool. They have very little overhead, with
/// only 1 atomic write before it's placed in the pool. (See [ThreadPool::spawn][`rayon::ThreadPool::spawn`])
pub fn go<F>(func: F)
where
    F: FnOnce() + Send + 'static,
{
    return unsafe {
        ACTIVE_THREADS.fetch_add(1, Ordering::Relaxed);
        POOL.spawn(|| {
            func();
            ACTIVE_THREADS.fetch_sub(1, Ordering::Relaxed);
        })
    };
}

/// Spins on the current thread until all green threads have completed.
pub fn await_threads() {
    while ACTIVE_THREADS.load(Ordering::Relaxed) != 0 {
        std::hint::spin_loop();
    }
}

/// Returns the number of hardware threads in use by the global thread pool.
pub fn num_threads() -> usize {
    unsafe { POOL.current_num_threads() }
}
