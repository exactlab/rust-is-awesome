extern crate libc; // defined in .toml
use std::mem;

#[derive(Debug)]
struct Vector<T: Copy> {
    len: u32,
    start: *mut T,
}

impl<T: Copy> Vector<T>{
    fn new() -> Self {
        unsafe {
            Self { 
                len: 0,
                start: libc::malloc(1*mem::size_of::<T>()) as *mut T
            }
        }
    }
}

fn main() {
    let vector = Vector::<i32>::new();
    println!("{:?}", vector)
}