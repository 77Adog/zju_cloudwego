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
    println!("hello world!");
}

pub async fn demo1() {
    let (tx, rx) = async_channel::bounded(1);
    async_std::task::spawn(demo2(tx, 2));
    println!("hello world: 1");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>, i: i32) {
    println!("hello world: {}", i);
    let _ = tx.send(()).await;
}

pub async fn test_demo() {
    // 可以修改测试规模
    let test_size = 100000;
    let mut vec_sender: Vec<async_channel::Sender<()>> = Vec::new();
    let mut vec_receiver: Vec<async_channel::Receiver<()>> = Vec::new();
    for _i in 0..test_size {
        let (tx, rx) = async_channel::bounded(1);
        vec_sender.push(tx);
        vec_receiver.push(rx);
    }
    for _i in 0..test_size {
        async_std::task::spawn(demo2(vec_sender.pop().unwrap(), _i));
    }
    for _i in 0..test_size {
        let _ = (vec_receiver.pop()).unwrap().recv().await;
    }
}