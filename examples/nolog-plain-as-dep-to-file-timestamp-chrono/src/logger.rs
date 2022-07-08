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

pub use nolog_plain::{crit, debug, error, info, trace, warn /* writelog */};
// We don't import writelog because we want to override it --- ^^^^^^^^

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
writelog { ( $msg:expr ) => { log($msg) } }
//                            ^^^
// function `log` instead of `println` macro.

use chrono::prelude::*;
use std::fmt::Arguments;
use std::fs::OpenOptions;
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
        // let time: DateTime<Local> = Local::now();
        let time: DateTime<Utc> = Utc::now();
        // UTC ----------- ^^^ -- ^^^
        let mut buf = BufWriter::new(file); 
        // [2022-07-10 06:49:33.646393648 UTC] DEBG: text 42,42,24 [src/main.rs 23:5]
        writeln!(buf, "[{time}] {msg}").ok();
        // Time -------- ^^^^
        buf.flush().ok();
    };
}
