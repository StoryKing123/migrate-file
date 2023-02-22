use chrono::prelude::*;
use clap::Parser;
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
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
    let path = String::from(file_path.clone());
    // println!("{:?}", args.clone());
    println!("input:{}", file_path);
    println!("output:{}", output_file);
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            let local: DateTime<Local> = Local::now();
            println!(
                "file changed, override the body to output file {}.[{}]",
                &output_file,
                local.format("%Y-%m-%d %H:%M:%S")
            );
            let mut src_file = File::open(&file_path).expect("");
            let mut dst_file = File::create(&output_file).expect("");
            io::copy(&mut src_file, &mut dst_file);
            println!("updated!");
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;
    watcher.watch(
        Path::new(String::from(path).as_str()),
        RecursiveMode::Recursive,
    )?;
    loop {}
    Ok(())
}
