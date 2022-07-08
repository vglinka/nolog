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

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
trace { ( $($msg:expr),* ) => { writelog!(format_args!("\x1B[34mTRCE:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
debug { ( $($msg:expr),* ) => { writelog!(format_args!("\x1B[36mDEBG:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
info { ( $($msg:expr),* )  => { writelog!(format_args!("\x1B[32mINFO:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
warn { ( $($msg:expr),* )  => { writelog!(format_args!("\x1B[33mWARN:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
error { ( $($msg:expr),* ) => { writelog!(format_args!("\x1B[31mERRO:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }
#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
crit { ( $($msg:expr),* )  => { writelog!(format_args!("\x1B[35mCRIT:\x1B[0m {} [{} {}:{}]", format_args!($($msg),*), file!(), line!(), column!())) } }

#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! trace { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! debug { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! info  { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! warn  { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! error { ( $($msg:expr),* ) => () }
#[rustfmt::skip] #[macro_export] #[cfg(not(debug_assertions))] macro_rules! crit  { ( $($msg:expr),* ) => () }

#[rustfmt::skip] #[macro_export] #[cfg(debug_assertions)] macro_rules!
writelog { ( $msg:expr ) => { println!("{}", $msg) } }
