//! Opaque trait for allowing custom opaque wrappers.
//! 
//! If your type implements this trait, then Rust supports it.

use alloc::boxed::Box;

pub trait DiplomatOpaque<T> {
    fn as_ptr(&mut self) -> *mut T;
}

impl<T> DiplomatOpaque<T> for Box<T> {
    fn as_ptr(&mut self) -> *mut T {
        &raw mut *self as *mut T
    }
}