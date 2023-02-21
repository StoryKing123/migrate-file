use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::Duration;

struct Args{
    input:String,
    output:String
}

fn main() -> Result<()> {
    let file_path = "test.js";
    let output_file = "output.js";
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            println!(
                "file changed, override the body to output file,{}",
                &output_file
            );
            let mut src_file = File::open(&file_path).expect("");
            let mut dst_file = File::create(output_file).expect("");
            io::copy(&mut src_file, &mut dst_file);
            println!("updated!");
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;
    watcher.watch(Path::new(file_path), RecursiveMode::Recursive)?;
    loop {
        // watcher
    }
    Ok(())
}
