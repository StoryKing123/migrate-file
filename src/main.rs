use chrono::prelude::*;
use clap::Parser;
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::exit;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(author,version,about,long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = args.input;
    let output_file = args.output;

    let copy_source_path = Path::new(&file_path).to_path_buf();
    let source_path = Path::new(&file_path);
    let copy_target_path = Path::new(&output_file).to_path_buf();

    let source_path_before_listening = copy_source_path.clone();
    let target_path_before_listening = copy_target_path.clone();

    println!("input:{}", file_path);
    println!("output:{}", output_file);
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            thread::sleep(Duration::from_millis(500));
            let local: DateTime<Local> = Local::now();
            println!(
                "file changed, override the body to output file {}.[{}]",
                &output_file,
                local.format("%Y-%m-%d %H:%M:%S")
            );
            copy_file(&copy_source_path, &copy_target_path);
            // copy_file(&source_path_before_listening.clone(), &copy_target_path);
            println!("updated!");
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    loop {
        if source_path.exists() && source_path.is_file() {
            println!("find file");
            copy_file(&source_path_before_listening, &target_path_before_listening);
            watcher.watch(source_path, RecursiveMode::NonRecursive)?;
            break;
        } else if source_path.is_dir() {
            println!("input can only be a file. not a dir.");
            exit(0x0100);
        } else {
            println!("no file");
        }
        thread::sleep(Duration::from_secs(1));
    }

    loop {
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn copy_file(source: &Path, target: &Path) {
    println!("start copy: {:?}", source);
    std::fs::copy(source, target);
}
