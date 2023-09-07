use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
// use std::task::RawWaker;
use std::task::Waker;
// use std::task::RawWakerVTable;
use std::task::Context;
use std::sync::Condvar;
use std::sync::Mutex;
use std::task::Wake;
use futures::future::BoxFuture;
use std::collections::VecDeque;


// fn dummy_waker() -> Waker {
//     static DATA: () = ();
//     unsafe {Waker::from_raw(RawWaker::new(&DATA, &VTABLE))}
// }

// const VTABLE: RawWakerVTable = RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);

// unsafe fn vtable_clone(_p: *const()) -> RawWaker {
//     RawWaker::new(_p, &VTABLE)
// }

// unsafe fn vtable_wake(_p: *const ()) {}

// unsafe fn vtable_wake_by_ref(_p: * const ()) {}

// unsafe fn vtable_drop(_p: * const ()) {}

enum State {
    Empty,
    Waiting,
    Notified,
}

struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

impl Signal {

    fn new() -> Signal {
        Signal { state: Mutex::new(State::Notified), cond: Condvar::new() }
    }
    
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("multiple wait");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }

    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

scoped_tls::scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_tls::scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);

pub fn block_on<F: Future>(future: F)-> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone()); 

    let mut cx = Context::from_waker(&waker);

    let runnable: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || {
            loop {
                if let std::task::Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = runnable.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        
        self.signal.notify();
    }
}