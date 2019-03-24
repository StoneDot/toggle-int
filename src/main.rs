#![feature(arbitrary_self_types)]

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

#[derive(Debug)]
struct SelfRef {
    ary: [i32; 2],
    ptr: NonNull<i32>,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(ary: [i32; 2]) -> Pin<Box<SelfRef>> {
        let data = SelfRef {
            ary,
            ptr: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(data);

        let ptr = NonNull::from(&boxed.ary[0]);
        unsafe {
            boxed.as_mut().get_unchecked_mut().ptr = ptr;
        }
        boxed
    }
    pub fn set_val(self: &mut Pin<Box<SelfRef>>, value: i32) {
        unsafe {
            *self.as_mut().get_unchecked_mut().ptr.as_mut() = value;
        }
    }
    pub fn toggle(self: &mut Pin<Box<SelfRef>>) {
        let ptr;
        if NonNull::from(&self.ary[0]) == self.ptr {
            ptr = NonNull::from(&self.ary[1]);
        } else {
            ptr = NonNull::from(&self.ary[0]);
        }
        unsafe {
            self.as_mut().get_unchecked_mut().ptr = ptr;
        }
    }
    pub fn get(&self) -> i32 {
        unsafe { *self.ptr.as_ref() }
    }
}

fn main() {
    let mut data = SelfRef::new([1, 2]);
    assert_eq!(1, data.get());
    data.set_val(100);
    assert_eq!(100, data.get());
    data.toggle();
    assert_eq!(2, data.get());
    data.toggle();
    assert_eq!(100, data.get());
}
