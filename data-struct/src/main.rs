extern crate libc; // defined in .toml
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use libc::c_void;

struct Vector<T: Copy> {
    len: u32,
    cap: u32,
    start: *mut T,
}

impl<T: Display + Debug + Copy> Vector<T>{
    fn new(cap: u32) -> Self {
        unsafe {
            Self { 
                len: 0,
                cap: cap,
                start: libc::malloc(
                    (cap as usize)*mem::size_of::<T>()
                ) as *mut T
            }
        }
    }

    fn grow(self: &mut Self) -> () {
        if self.len == self.cap {
            unsafe{
                self.start = libc::realloc(
                    self.start as *mut c_void,
                    ((self.cap * 2) as usize) * mem::size_of::<T>()
                ) as *mut T
            }
            self.cap = self.cap * 2
        }
    }

    fn push(self: &mut Self, element: T) -> &Self {
        self.grow();
        unsafe{
            *(self.start.offset(self.len as isize)) = element;
        }
        self.len += 1;
        self
    }

    fn pop(self: &mut Self) -> Option<T> {
        if self.len < 1 {
            return None;
        }
        self.len -= 1;
        unsafe{
            Some(*(self.start.offset(self.len as isize)))
        }
    }
}

impl<T: Copy> Drop for Vector<T>{
    fn drop(self: &mut Self){
        unsafe {
            println!("not free");
            libc::free(self.start as *mut c_void);
            println!("free");
        }
    }
}


impl<T: Copy + Display> Display for Vector<T>{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut value: T;
        for i in 0..self.len {
            unsafe{
                value = *(self.start.offset(i as isize));
            }
            write!(fmt, "{} ", value)?;
        }
        write!(fmt, "\n")
    }
}

fn main(){
    {
        let mut vector = Vector::<i32>::new(1);

        for i in 0..33 {
            vector.push(i);
            println!("{} {:?}", vector, vector.start);
        } 

        for i in 0..vector.len {
            let popped = vector.pop().unwrap();
            println!("Popped {:?}", popped);
            println!("{} {:?}", vector, vector.start);
        } 
        


    }
}