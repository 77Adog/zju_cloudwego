mod runtime;
mod task;


fn main() {
    // 单线程多任务测试
    runtime::block_on(task::demo1());
    // // 单线程单任务测试
    println!("Run the demo test that waits for network for 10s. You can use htop to check the CPU usage.");
    runtime::block_on(task::demo());
    if false {
        // 消除没有运行test程序的warning所以有这个设计，若要验证输出测试的输出正确，可以将if条件改为true随后运行查看
        runtime::block_on(task::test_demo());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // 使用test进行时间测试
    #[test]
    fn test_runtime() {
        runtime::block_on(super::task::test_demo());
    }
}

