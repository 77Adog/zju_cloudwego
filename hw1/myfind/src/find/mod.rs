
use std::path::Path;
use regex::Regex;

mod walk_tree;

pub fn find<P: AsRef<Path>>(
    root: P, 
    regex: &Regex,
    is_v: bool // 判断是不是-v参数
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    log::info!("进入find模块");
    let mut matches = Vec::new();
    walk_tree::walk_tree(root.as_ref(), regex, is_v, &mut matches)?;
    Ok(matches)
}

