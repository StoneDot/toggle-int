use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

#[derive(Debug)]
struct SelfRef {
    ary: [i32; 2],
    ptr: NonNull<i32>,
    _pin: PhantomPinned,
}

pub struct ToggleInt {
    inner: Pin<Box<SelfRef>>,
}

impl ToggleInt {
    fn new(ary: [i32; 2]) -> ToggleInt {
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
        ToggleInt { inner: boxed }
    }
    pub fn set(&mut self, value: i32) {
        let target = &mut self.inner;
        unsafe {
            *target.as_mut().get_unchecked_mut().ptr.as_mut() = value;
        }
    }
    pub fn toggle(&mut self) {
        let target = &mut self.inner;
        let ptr;
        if NonNull::from(&target.ary[0]) == target.ptr {
            ptr = NonNull::from(&target.ary[1]);
        } else {
            ptr = NonNull::from(&target.ary[0]);
        }
        unsafe {
            target.as_mut().get_unchecked_mut().ptr = ptr;
        }
    }
    pub fn get(&self) -> i32 {
        unsafe { *self.inner.ptr.as_ref() }
    }
}

fn main() {
    let mut data = ToggleInt::new([1, 2]);
    assert_eq!(1, data.get());
    data.set(100);
    assert_eq!(100, data.get());
    data.toggle();
    assert_eq!(2, data.get());
    data.toggle();
    assert_eq!(100, data.get());
}
