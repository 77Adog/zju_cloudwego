use reqwest::{header::CONTENT_TYPE, Client};
use std::thread::spawn;

#[tokio::main]
async fn main() {
    // 创建http客户端
    let client = reqwest::Client::new();

    // 删除需要测试的key
    let body = "key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(del.status(), 200);

    /*================ 测试set ===================*/
    println!("Test set operations:");

    let body = "key=foo&val=123";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    
    assert_eq!(set.text().await.unwrap(), "Set success!".to_string());

    let body = "key=foo&val=123";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    // 重复设置
    assert_eq!(set.text().await.unwrap(), "The key: foo is already in the database".to_string());

    println!("Test set operation successfully!");

    /*================ 测试get ===================*/
    println!("Test get operations:");

    let get = client
        .get("http://localhost:3000/get/foo")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "123");

    let get = client
        .get("http://localhost:3000/get/foo123")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // get一个不存在的key对应的值
    assert_eq!(get, "The key: foo123 is not in the database");

    println!("Test get operation successfully!");

    /*================ 测试del ===================*/

    println!("Test del operations:");

    // 删除上述插入的key
    let body = "key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(del, "Del success!");

    // 删除一个不存在的key
    let body = "key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(del, "The key: foo is not found in the database");

    println!("Test del operation successfully!");

    /*================ 测试ping ===================*/

    println!("Test ping operations");

    // 测试没有信息的ping
    let pong = client
        .get("http://localhost:3000/ping")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pong, "pong");

    // 测试有信息的ping
    let pong = client
        .get("http://localhost:3000/ping/foo")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pong, "foo");

    println!("Test ping operation successfully!");

    /*================ 测试subscribe和publish ===================*/

    println!("Test publish and subscribe operations:");

    // 创建一个线程，在十秒后发送publish，在本线程创建一个subscriber
    let _m = tokio::spawn(publish(client.clone()));

    let body = "channel_name=foo";
    let set = client
        .post("http://localhost:3000/subscribe")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    assert_eq!(set, "123");

    println!("Test publish and subscribe operations successfully!");

    println!("Test success!");
}

async fn publish(client: Client) -> String {
    let handle = spawn(|| {
        let d = std::time::Duration::from_secs(10);
        std::thread::sleep(d);
    });
    handle.join().unwrap();
    let body = "channel_name=foo&value=123";
    let set = client
        .post("http://localhost:3000/publish")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    set
}