use std::mem;
use std::ptr::{self, NonNull};

pub struct Vec<T> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec {
            ptr: Unique(None),
            len: 0,
            cap: 0,
        }
    }
}

struct Unique<T>(Option<NonNull<T>>);

unsafe impl<T: Send> Send for Unique<T> {}
unsafe impl<T: Sync> Sync for Unique<T> {}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn it_works() {
        let vec = Vec::<u8>::new();
    }
}
