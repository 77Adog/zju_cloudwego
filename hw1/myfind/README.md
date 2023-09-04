# myfind

编译前请配置好rustup工具链

使用如下命令编译
```
cargo build --release
```

使用如下命令即可得到find命令使用的方式
```
./target/release/myfind
```

使用格式为
```
./target/release/myfind <目标目录数量> <目标目录1> <目标目录2>.... <要搜索的正则表达式的个数> <要搜索的正则表达式1> <要搜索的正 则表达式2>... [Options]
Options:
-v: 列出所有遍历到的文件，匹配为绿色字体，不匹配为红色字体，最后会用蓝色字体展示匹配的文件
```

程序会将所有目标目录中，满足任意一个正则表达式的文件进行输出，输出前会进行去重排序

例如
```
./target/release/myfind 2 ~/Desktop ~/Document 2 nvme gcc -v
```

注意输入正确的路径和正则表达式，不然会有对应的错误信息

程序运行时会输出日志信息，日志信息将会保存在target/log目录下