use std::cell::Cell;
use std::mem;
use std::ptr;
use std::ops::Deref;
use std::fmt;

struct RcBox<T> {
    strong: Cell<usize>,
    value: T
}

pub struct Rc<T> {
    _ptr: *mut RcBox<T>
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        let mut rc_box  = Box::new(
            RcBox { strong: Cell::new(1), value: value }
        );
        let rc = Rc {
            _ptr: &mut *rc_box
        };
        mem::forget(rc_box);
        rc
    }

    pub fn strong_count(&self) -> usize {
        self.get_strong()
    }

    fn inner(&self) -> &RcBox<T> {
        unsafe {
            &*self._ptr
        }
    }

    fn get_strong(&self) -> usize {
        self.inner().strong.get()
    }

    fn dec_strong(&self) { 
        self.inner().strong.set(self.get_strong() - 1); 
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        self.inner().strong.set(self.get_strong() + 1);
        Rc {_ptr: self._ptr}
    }
}

impl<T> Drop for Rc<T> {
    
    fn drop(&mut self) {
        // println!("shabi");
        self.dec_strong();
        if self.get_strong() == 0 {
            unsafe {
                mem::drop(ptr::read(&(*self._ptr).value));
            }
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T: fmt::Display> fmt::Display for Rc<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

