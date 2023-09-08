use std::{future::Future, time::Duration};

pub struct Demo;

impl Future for Demo {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("hello world");
        std::task::Poll::Ready(())
    }
}

pub async fn demo() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(10));
        tx.send_blocking(())
    });
    let _ = rx.recv().await;
    println!("hello world: ");
}

pub async fn demo1() {
    let (tx, rx) = async_channel::bounded(1);
    let (tx1, rx1) = async_channel::bounded(1);
    super::runtime::spawn(demo2(tx, 2));
    super::runtime::spawn(demo2(tx1, 3));
    println!("hello world: 1");
    let _ = rx.recv().await;
    let _ = rx1.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>, i: i32) {
    println!("hello world: {}", i);
    let _ = tx.send(()).await;
}

pub async fn test_demo() {
    // 可以修改测试规模
    let test_size = 10000;
    let mut vec_sender: Vec<async_channel::Sender<()>> = Vec::new();
    let mut vec_receiver: Vec<async_channel::Receiver<()>> = Vec::new();
    for _i in 0..test_size {
        let (tx, rx) = async_channel::bounded(1);
        vec_sender.push(tx);
        vec_receiver.push(rx);
    }
    for i in 0..test_size {
        super::runtime::spawn(test_single(vec_sender.pop().unwrap(), i) );
    }
    for _i in 0..test_size {
        let _ = (vec_receiver.pop()).unwrap().recv().await;
    }
}

async fn test_single(_t: async_channel::Sender<()>, _test_id: i32) {
    async {}.await;
    let mut _num = 0;
    for i in 0..1000000 {
        _num += i;
        _num -= i;
    }
    async {}.await;
}