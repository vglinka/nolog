/* ----------------------------------------------------------------------------
MIT License

Copyright (c) 2022 Vadim Glinka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
---------------------------------------------------------------------------- */

#[rustfmt::skip] #[macro_export] macro_rules!
trace { ( $($msg:expr),* ) => { writelog!(format_args!("TRCE: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
debug { ( $($msg:expr),* ) => { writelog!(format_args!("DEBG: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
info { ( $($msg:expr),* )  => { writelog!(format_args!("INFO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
warn { ( $($msg:expr),* )  => { writelog!(format_args!("WARN: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
error { ( $($msg:expr),* ) => { writelog!(format_args!("ERRO: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] macro_rules!
crit { ( $($msg:expr),* )  => { writelog!(format_args!("CRIT: {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }

#[rustfmt::skip] #[macro_export] macro_rules!
writelog { ( $msg:expr ) => { log($msg) } }
//                            ^^^
// function `log` instead of `println` macro.

use std::fmt::Arguments;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct LogFile;

impl LogFile {
    pub fn path() -> PathBuf {
        PathBuf::from("log.txt")
    }
}

#[rustfmt::skip]
pub fn log(msg: Arguments) {
//              ^^^^^^^^^ std::fmt::Arguments
    let path = LogFile::path();
    let open_file = |path| { OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
    };
    if let Ok(file) = open_file(path) {
        let mut buf = BufWriter::new(file);
        writeln!(buf, "{msg}").ok();
        buf.flush().ok();
    };
}

fn main() -> io::Result<()> {
    let path = LogFile::path();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        //^^^^^^^ truncate the file to 0 length if it already exists.
        .open(&path)?;

    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("-- In {path:?} --");
    println!("{contents}");
    
    /* Output:
    -- In "log.txt" --
    TRCE: text 42,42,42 [examples/to-file-debug-plus-release.rs 84:5]
    DEBG: text 42,42,24 [examples/to-file-debug-plus-release.rs 85:5]
    INFO: text 42,24,42 [examples/to-file-debug-plus-release.rs 86:5]
    WARN: text 42,a,422 [examples/to-file-debug-plus-release.rs 87:5]
    ERRO: text 42,42,42 [examples/to-file-debug-plus-release.rs 88:5]
    CRIT: text 42,42,42 [examples/to-file-debug-plus-release.rs 89:5] 
    */

    Ok(())
}
