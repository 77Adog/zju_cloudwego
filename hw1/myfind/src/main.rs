
use regex::Regex;
use std::env;
use std::process;
use colored::*;

use log;
use tracing_appender;

// 引入我自己设计的find模块
mod find;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 创建tracing
    let file_appender = tracing_appender::rolling::hourly("target/log/", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();

    log::info!("程序开始");

    // 参数1： 搜索目录; 参数 2: 要搜索的正则表达式.
    if args.len() < 3 {
        eprintln!("使用方式: {} <目标目录数量> <目标目录1> <目标目录2>.... <要搜索的正则表达式的个数> <要搜索的正则表达式1> <要搜索的正则表达式2>... [Options]", args[0]);
        eprintln!("Options: ");
        eprintln!("-v: 列出所有遍历到的文件，匹配为绿色字体，不匹配为红色字体，最后会用蓝色字体展示匹配的文件");
        log::info!("程序异常退出");
        process::exit(1);
    }

    log::info!("获取目标路径数量");
    // 获得目标路径的数量
    let path_num = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("目标目录数量不是一个整型数据 {}, {}", args[1], err);
            log::info!("程序异常退出");
            process::exit(1);
        }
    };

    log::info!("获取目标路径");
    // 获得所有目标路径
    let mut paths: Vec<String> = Vec::new();
    for i in 2..(2 + path_num) {
        if i >= args.len() {
            eprintln!("目标路径的数量过少");
            log::info!("程序异常退出");
            process::exit(1);
        }
        paths.push(args[i].clone());
    }

    log::info!("获取正则表达式数量");
    // 获得正则表达式的数量
    let pattern_num = match args[path_num + 2].parse::<usize>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("第三个参数不是一个整型数据 {}, {}", args[2], err);
                log::info!("程序异常退出");
                process::exit(1);
            }
    };

    log::info!("获取正则表达式");
    // 获得所有输入的正则表达式
    let regexs: Vec<Regex> = match get_patterns(path_num + 3, 3 + path_num + pattern_num, &args) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("发生错误：{}", err);
            log::info!("程序异常退出");
            process::exit(1);
        }
    };

    let mut index:usize = 3 + path_num + pattern_num;

    log::info!("判断是否有-v等可选参数");
    // 当有多余参数时
    let mut is_v:bool = false; // 判断是不是-v参数
    while index < args.len() {
        if args[index].eq("-v") || args[index].eq("-V") {
            // 此时发现-v参数
            is_v = true;
        }
        index = index + 1;
    }

    log::info!("寻找匹配项");
    // 找到所有的匹配项
    let all_matches = match find_all_match(&regexs, &paths, is_v) {
        Ok(matches) => matches,
        Err(err) => {
            eprintln!("发生错误：{}", err);
            log::info!("程序异常退出");
            process::exit(1);
        }
    };

    log::info!("排序去重");


    // 排序去重
    let final_matches = sort_remove_dup(all_matches);

    log::info!("输出结果");
    if final_matches.is_empty() {
        println!("未找到匹配项。");
    } else {
        println!("找到以下匹配项：");
        for file in final_matches {
            println!("{}", file.blue());
        }
    }

    log::info!("程序执行结束");
}

// 从参数中获得所有的正则表达式
fn get_patterns(begin: usize, end: usize, args: &Vec<String>) -> Result<Vec<Regex>, Box<dyn std::error::Error>> {
    let mut regexs: Vec<Regex> = Vec::new();
    for i in begin..end {
        if i >= args.len() {
            return Err("正则表达式的数量少于参数指定的数量".into());
        }
        let pattern = &args[i];
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(err) => {
                eprintln!("无效的正则表达式 '{}': {}", pattern, err);
                log::info!("程序异常退出");
                process::exit(1);
            }
        };
        regexs.push(regex);
    }
    Ok(regexs)
}

// 找到所有的满足条件的文件
fn find_all_match(regexs: &Vec<Regex>, paths: &Vec<String>, is_v: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut all_matches: Vec<String> = Vec::new();
    for path in paths {
        for regex in regexs {
            let files = find::find(path, &regex, is_v)?;
            for file in files {
                all_matches.push(file);
            }
        }
    }
    Ok(all_matches)
}

// 排序去重
fn sort_remove_dup(mut all_matches: Vec<String>) -> Vec<String> {
    let mut final_matches: Vec<String> = Vec::new();
    all_matches.sort_unstable();
    for file in all_matches {
        match final_matches.last() {
            Some(last_file) => {
                if !last_file.eq(&file) {
                    final_matches.push(file);
                }
            }
            None => {
                final_matches.push(file);
            }
        }
    }
    final_matches
}


