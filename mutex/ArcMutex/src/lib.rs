use std::sync::{Arc, LockResult, Mutex, MutexGuard, TryLockResult};

pub struct ArcMutex<T>(Arc<Mutex<T>>);

impl<T> ArcMutex<T> {
    pub fn new(t: T) -> ArcMutex<T> {
        ArcMutex(Arc::new(Mutex::new(t)))
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        self.0.lock()
    }
    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        self.0.try_lock()
    }

    pub fn clone(&self) -> ArcMutex<T> {
        ArcMutex(Arc::clone(&self.0))
    }
}
