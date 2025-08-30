mod file_data;
mod args;

use std::{fs, path::PathBuf, str::FromStr, sync::Arc};

use chrono::{DateTime, Local};
use clap::{Parser};

use crate::file_data::FileData;
use threadpool::ThreadPool;
use regex::Regex;

fn main() {
    let args = args::Args::parse();
    if args.content.eq("\0") && args.regex.eq("\0"){
        eprintln!("Error: must provide --content or --regex flags");
        return;
    }

    if let Err(e) = find(args.paths, args.threads, &args.content, &args.regex) {
            eprintln!("Error: {e}");
    };
}

pub fn find(paths: Vec<PathBuf>, threads: usize, content: &str, regex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = ThreadPool::new(threads);
    let arc_clone:Arc<ThreadPool> = Arc::new(pool);

    println!("{}", FileData::begin());

    if !content.eq("\0"){
        let arc_content:Arc<String> = Arc::new(String::from_str(content).ok().unwrap());

        for path in paths {
            if let Err(e) = search_recursively(&path, arc_clone.clone(), arc_content.clone()) {
                eprintln!("Error: {e}");
            };
        }
    } else {
        let arc_regex:Arc<String> = Arc::new(String::from_str(regex).ok().unwrap());

        for path in paths {
            if let Err(e) = search_recursively_regex(&path, arc_clone.clone(), arc_regex.clone()) {
                eprintln!("Error: {e}");
            };
        }
    }

    arc_clone.join();
    Ok(())
}

fn search_recursively(path: &PathBuf, pool: Arc<ThreadPool>, content: Arc<String>) -> Result<(), Box<dyn std::error::Error>> {
    for file in fs::read_dir(path)?.flatten() {

        let file_name: String = file.file_name().to_string_lossy().to_string();

        let path: String = file.path().to_string_lossy().to_string();

        let last_change: DateTime<Local> = DateTime::from(file.metadata().ok().unwrap().modified().ok().unwrap());
        let last_change_string: String = last_change.format("%Y-%m-%d %H:%M:%S").to_string();

        let content: Arc<String> = content.clone();

        if file_name.trim().to_ascii_lowercase().as_str().contains(content.trim().to_ascii_lowercase().as_str()){
            let file_data: FileData = FileData::new(file_name,
                                path, 
                                last_change_string);
            output(&file_data);
        }

        if file.file_type().ok().unwrap().is_dir(){
            let pool_clone = Arc::clone(&pool);
            pool.execute(move || {
                let _ = search_recursively( &file.path(), Arc::clone(&pool_clone), Arc::clone(&content));
            });
        }
    }

    Ok(())
}

fn search_recursively_regex(path: &PathBuf, pool: Arc<ThreadPool>, regex: Arc<String>) -> Result<(), Box<dyn std::error::Error>> {
    let pattern: Regex = Regex::new(&regex).unwrap();
    for file in fs::read_dir(path)?.flatten() {

        let file_name: String = file.file_name().to_string_lossy().to_string();

        let path: String = file.path().to_string_lossy().to_string();

        let last_change: DateTime<Local> = DateTime::from(file.metadata().ok().unwrap().modified().ok().unwrap());
        let last_change_string: String = last_change.format("%Y-%m-%d %H:%M:%S").to_string();

        let content: Arc<String> = regex.clone();

        if pattern.find(&file_name).is_some(){
            let file_data: FileData = FileData::new(file_name,
                                path, 
                                last_change_string);
            output(&file_data);
        }

        if file.file_type().ok().unwrap().is_dir(){
            let pool_clone = Arc::clone(&pool);
            pool.execute(move || {
                let _ = search_recursively_regex( &file.path(), Arc::clone(&pool_clone), Arc::clone(&content));
            });
        }
    }

    Ok(())
}

fn output(file_data: &FileData){
    println!("{}", file_data.to_string());
}