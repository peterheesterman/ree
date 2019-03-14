use std::fs;
use std::env;
use colored::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // if there are too many arguments panic

    list_paths(filename.to_string(), 0);
}

fn padding_for_depth(depth: i32) -> String {
    format!("{}", (0..2*depth).map(|_| " ").collect::<String>())
}

fn list_paths(path: String, depth: i32) {
    let folder_path = Path::new(&path);
    let folder_parent_path = folder_path.parent().unwrap();
    println!(
        "{}/{}",
        padding_for_depth(depth),
        folder_path.strip_prefix(folder_parent_path.to_str().unwrap()).unwrap().to_str().unwrap()
            .blue()
            .bold()
    );
    let depth = depth + 1;
    
    let sub_paths = fs::read_dir(&path).unwrap();
    for path in sub_paths {
        let path = format!("{}", path.unwrap().path().display());
        let metadata = fs::metadata(path.clone()).unwrap();
        if metadata.is_dir() {
            list_paths(String::from(path), depth)
        } else {
            let file_name = Path::new(&path).file_name().unwrap();
            println!("{}{}",  padding_for_depth(depth), file_name.to_str().unwrap().green());
        };
    }
}