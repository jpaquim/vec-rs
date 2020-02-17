use std::mem;
use std::ptr::{self, NonNull};

pub struct Vec<T> {
    ptr: Option<NonNull<T>>,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec {
            ptr: None,
            len: 0,
            cap: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn it_works() {
        let vec = Vec::<u8>::new();
    }
}
