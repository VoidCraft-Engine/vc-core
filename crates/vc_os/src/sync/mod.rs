#[cfg(not(target_has_atomic = "ptr"))]
compile_error!("Platforms without atomic pointers are currently not supported.");

pub mod atomic;
pub use alloc::sync::{Arc, Weak};

mod once_flag;
pub use once_flag::OnceFlag;

crate::cfg::switch! {
    crate::cfg::std => {
        pub use std::sync::{
            Barrier, BarrierWaitResult, LazyLock, LockResult,
            Mutex, MutexGuard, Once, OnceLock, OnceState,
            PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard,
            TryLockError, TryLockResult, mpsc,
            Condvar, WaitTimeoutResult,
        };
    }
    _ => {
        compile_error!("This platform is not supported");
    }
}
