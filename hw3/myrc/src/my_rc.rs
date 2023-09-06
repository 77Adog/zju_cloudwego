use std::ops::Deref;
use std::fmt;
use std::sync::Arc;

pub struct Rc<T> {
    _ptr: Arc<T>
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        Rc {
            _ptr: Arc::new(value)
        }
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self._ptr)
    }


}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        Rc {_ptr: self._ptr.clone()}
    }
}

impl<T> Drop for Rc<T> {
    
    fn drop(&mut self) {
        
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self._ptr.deref()
    }
}

impl<T: fmt::Display> fmt::Display for Rc<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self._ptr.deref(), f)
    }
}

