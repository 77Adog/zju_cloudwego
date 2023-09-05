# 第二次作业

- 作业日期：2023年9月5日

- 该目录下有三个文件夹，分别对应三个项目的工程文件

- `buffer/` 中包含了课件第11页的练习工程文件，实现了一个`Buffer<T>`类，其中有成员`Vec<T>`，并实现了一个方法sum，main.rs中含有测试脚本供简单测试

- `closure/` 中包含了课件第26页的练习的工程文件，将一个内容为`['a', 'b', 'c', 'd', 'e']`的`Vec<char`通过闭包和迭代器生成了一个内容为`['b', 'c', 'd', 'e', 'f']`。

- `strcmp/` 中包含了课件第17页的练习的工程文件，实现了`fn compareString(x: &str, y: &str) -> bool`函数，判断字符串x是否比y字符串字典序更大，在main.rs中实现了一个测试脚本进行简单测试

- 在本github仓库中设置好了github action，能实现在每次push新的commit后进行hw1的自动编译，将hw2的buffer和strcmp两个工程进行编译和运行测试脚本，将hw2的closure工程进行编译运行，因为验证是否运行正确的脚本在main函数中，直接运行即可。自动编译的脚本在仓库根目录的`.github/workflows/main.yml`中。