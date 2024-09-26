use core::{cell::UnsafeCell, sync::atomic::{AtomicBool,Ordering}};
use core::arch::asm;

pub struct Mutex<T>{
    locked:AtomicBool,
    data:UnsafeCell<T>
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T>{
    pub const fn new(data:T)->Self{
        Mutex{
            locked:AtomicBool::new(false),
            data:UnsafeCell::new(data)
        }
    }
    pub fn lock(&self)->MutexGuard<T>{
        while self.locked.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err(){
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        unsafe { asm!("fence rw, rw"); }
        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { asm!("fence rw, rw"); }
        self.mutex.locked.store(false, Ordering::Release);
    }
}