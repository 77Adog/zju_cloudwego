# 第四次作业

> 本次作业实现了一个单线程支持多任务的runtime，调用`runtime::block_on()`接口即可将异步函数放入runtime进行执行，在`task.rs`中实现了课件中并发输出hello world的测试，并在`main`函数中使用`block_on`接口进行运行，对课件中相关现象进行了验证。

> 本次作业还设计了一个测试，能够并发打印多次hello world，`main`函数中有个if，将其判断条件改为true再运行即可看到现象。使用`cargo test`可以获得并发打印如此多次hello world所需要的时间。