// This example is licensed under the CC0 license.
// https://creativecommons.org/share-your-work/public-domain/cc0
//
// This means that you can use the code from this example in your projects
// without any restrictions or attribution.

use std::fs::OpenOptions;
use std::io::{self, Read};
use std::path::PathBuf;

#[macro_use]
extern crate nolog;

// use `cargo run --features trace`

fn main() -> io::Result<()> {
    let path = PathBuf::from("log.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        //^^^^^^^ truncate the file to 0 length if it already exists.
        //.append(true)
        .open(&path)?;
        
    // Initialization
    // Don't use macros like `debug!("msg");` before initialization.
    logfile!(file);
    
    trace!("Hello from file!");
        
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("\n      -- In {path:?} --");
    println!("{contents}");
    
    Ok(())
}
