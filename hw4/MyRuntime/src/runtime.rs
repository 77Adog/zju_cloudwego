use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
// use std::task::RawWaker;
use std::task::Waker;
// use std::task::RawWakerVTable;
use std::task::Context;
use std::sync::Condvar;
use std::task::Wake;
use std::sync::Mutex;
use futures::FutureExt;
use futures::future::LocalBoxFuture;
use std::collections::VecDeque;

use rand::Rng;


enum State {
    Empty,
    Waiting,
    Notified,
}

struct Signal {
    state: std::sync::Mutex<State>,
    cond: Condvar,
}

impl Signal {

    fn new() -> Signal {
        Signal { state: std::sync::Mutex::new(State::Notified), cond: Condvar::new() }
    }
    
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {*state = State::Empty; },
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
scoped_tls::scoped_thread_local!(static RUNNABLE: Arc<TwoVD>);

struct TwoVD {
    vd1: Mutex<VecDeque<Arc<Task>>>,
    vd2: Mutex<VecDeque<Arc<Task>>>,
}

pub fn block_on<F: Future>(future: F)-> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone()); 

    let mut cx = Context::from_waker(&waker);

    let runnable: Arc<TwoVD> = Arc::new(TwoVD { vd1: Mutex::new(VecDeque::new()), vd2: Mutex::new(VecDeque::new()) });
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || {
            loop {
                if let std::task::Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                // 创建一个线程，将所有的附属task进行多线程处理
                let runnable1 = runnable.clone();
                let handle = std::thread::spawn(move || {
                    while let Some(task) = (*runnable1).vd2.lock().unwrap().pop_front() {
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        let _poll_result = task.future.borrow_mut().as_mut().poll(&mut cx);
                    }
                });
                while let Some(task) = (*runnable).vd1.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _poll_result = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                // 等待多线程执行结束
                handle.join().unwrap();
                signal.wait();
            }
        })
    })
}

struct Task {
    future: RefCell<LocalBoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        let mut rng =rand::thread_rng();
        // 生成随机数，由随机数决定wake以后进入哪个运行队列
        let num: usize = rng.gen();
        match num % 2 {
            0 => {RUNNABLE.with(|runnable| (*runnable).vd1.lock().unwrap().push_back(self.clone()));},
            _ => {RUNNABLE.with(|runnable| (*runnable).vd2.lock().unwrap().push_back(self.clone()));}
        }
        // RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

pub fn spawn<F: Future<Output = ()> + 'static>(fut: F) {
    let t = Arc::new(Task {
        future: RefCell::new(fut.boxed_local()),
        signal: Arc::new(Signal::new()),
    });
    // RUNNABLE.with(|runnable| {
    //     let mut task_queue = runnable.lock().unwrap();
    //     task_queue.push_back(t);
    // });
    let mut rng =rand::thread_rng();
    // 生成随机数，由随机数决定wake以后进入哪个运行队列
    let num: usize = rng.gen();
    match num % 2 {
        0 => {RUNNABLE.with(|runnable| (*runnable).vd1.lock().unwrap().push_back(t));},
        _ => {RUNNABLE.with(|runnable| (*runnable).vd2.lock().unwrap().push_back(t));}
    }
}