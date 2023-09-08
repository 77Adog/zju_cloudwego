# 第三次作业

- 作业日期：2023年9月6日

该目录下有三个文件夹：
- HashMapMacro: 关于创建hashmap的宏的工程文件，实现了相关宏以及写了相关测试用例，可以使用`cargo test`对其进行测试
- mystack: 关于使用RefCell实现了一个自己的栈，支持push和pop，实现了相关测试，可以使用`cargo test`对其进行测试
- myrc: 关于自己实现的rc智能指针实现的功能，目前只想得到用NonNull和arc的方法，当前交的作业中，my_rc1.rs中为NonNull实现的方法，my_rc.rs中为使用arc的方法。该工程中书写了测试文件以及主程序，同时对两种实现方法进行了使用和测试，可以使用`cargo test`进行测试，可以使用`cargo run`运行。