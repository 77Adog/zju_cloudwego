# 第六次作业

## 编译运行的方法

```shell
cd mini-redis/
cargo run --bin server
```

```shell
cd http/
cargo run
```

```shell
cd http /
cargo run --example demo
```

即可完成测试

## 手动测试的方法

```shell
cd mini-redis/
cargo run --bin server
```

```shell
cd http/
cargo run
```

- 使用ip:3000/set方法进入设置网页
- 使用ip:3000/get/:key查找相关的key值
- 使用ip:3000/del进入删除网页
- 使用ip:3000/ping，或ip:3000/ping/:msg进行ping操作
- 使用ip:3000/subscribe方法进入网页注册
- 使用ip:3000/publish方法进入网页发布