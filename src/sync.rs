//! Minimal synchronization primitives for `no_std` kernels.
//!
//! These primitives are intentionally simple and are meant as building blocks.
//! They are *not* a full-featured synchronization library.
//!
//! ## Important caveats
//!
//! - **No fairness guarantees**: spin locks may starve.
//! - **Not preemption-safe by themselves**: if your kernel is preemptive or can be interrupted,
//!   you may need to disable interrupts or otherwise ensure lock-holding sections are safe.
//! - **Single-core vs multi-core**: on SMP systems, these use atomics and are safe provided your
//!   platform's memory model matches Rust's atomic semantics.

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

/// A simple spin lock.
///
/// The lock is acquired via a compare-exchange loop and released on guard drop.
///
/// ## Safety
///
/// This type is `Send`/`Sync` when `T: Send` and relies on correct atomic behavior.
/// As with any spin lock, make sure lock hold times are short.
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Creates a new spin lock protecting `value`.
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Acquires the lock and returns a guard.
    ///
    /// This spins until the lock becomes available.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinLockGuard { lock: self }
    }
}

/// RAII guard returned by [`SpinLock::lock`].
///
/// Releases the lock when dropped.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// A minimal, spin-based `Once` initializer.
///
/// This type is useful for global singletons in `no_std` environments.
/// It supports one-time initialization with `call_once`.
///
/// ## State machine
///
/// - `0`: uninitialized
/// - `1`: initialization in progress
/// - `2`: initialized
pub struct Once<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Default for Once<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send + Sync> Sync for Once<T> {}
unsafe impl<T: Send> Send for Once<T> {}

impl<T> Once<T> {
    /// Creates a new uninitialized cell.
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(0),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Returns `true` if the value has been initialized.
    pub fn is_initialized(&self) -> bool {
        self.state.load(Ordering::Acquire) == 2
    }

    /// Returns a reference to the value if initialized.
    pub fn get(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    /// Initializes the cell with `init` at most once and returns a reference to the stored value.
    ///
    /// If another core is currently initializing, this method will spin until initialization
    /// completes.
    pub fn call_once(&self, init: impl FnOnce() -> T) -> &T {
        if self.is_initialized() {
            return unsafe { self.get_unchecked() };
        }

        if self
            .state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let v = init();
            unsafe {
                (*self.value.get()).write(v);
            }
            self.state.store(2, Ordering::Release);
            unsafe { self.get_unchecked() }
        } else {
            while !self.is_initialized() {
                core::hint::spin_loop();
            }
            unsafe { self.get_unchecked() }
        }
    }

    /// Returns a reference to the stored value without checking initialization.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that initialization has completed.
    unsafe fn get_unchecked(&self) -> &T {
        unsafe { &*(*self.value.get()).as_ptr() }
    }
}
