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

#[macro_use] 
pub mod logger_setup {
    #[macro_export]
    #[cfg(feature = "custom_writelog_inner")] macro_rules!
    writelog_inner { ( $msg:expr ) => {
            eprintln!("{}", $msg); // write to stderr
            tofile_writelog_inner_helper!($msg); // write to a file
        }
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

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
    
    eprintln!("\n-- From eprintln: --");
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();
    
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("\n-- In {path:?} --");
    println!("{contents}");
    
    Ok(())
}

