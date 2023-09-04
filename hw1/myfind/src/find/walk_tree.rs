
use std::fs;
use colored::*;
use std::path::Path;
use regex::Regex;

pub fn walk_tree (
    dir: &Path,
    regex: &Regex,
    is_v: bool,
    matches: &mut Vec<String>
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, is_v, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                    if is_v {
                        println!("{}", filename.green());
                    }
                } else if is_v {
                    println!("{}", filename.red());
                }
            }
        }
    } 
    else {
        return Err("目标路径存在无效路径".into());
    }
    Ok(())
}