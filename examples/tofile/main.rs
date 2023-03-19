// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

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
