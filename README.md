## nolog logger

20 lines of code with 0 deps.

- Colored output.
- Support for named format arguments `info!("{line_count} lines.");`.
- Displays the location of the code `[src/main.rs 15:5]`.
- Easy to add timestamp `[2022-07-10 06:49:33.646361181 UTC]`.
- Automatically disabled in the release build `cargo run --release`.
  You can modify the code to disable this.
- Same syntax as `log` crate. As the project grows, it is possible
  to migrate to an advanced logger (using the `log` crate facade)
  without changing the code.
- Can be built into the project directly instead of as a dependency.
- Uses Rust's built-in macros.
- Easy to modify to redirect log output (to `stderr` or `file`).
- MIT License.

![nolog](https://raw.githubusercontent.com/vglinka/nolog/main/assets/term.png)

## Examples

Download the examples below:

```sh
git clone https://github.com/vglinka/nolog
cd ./nolog/
```
```rust
cargo run --example base
cargo run --example to-stderr
cargo run --example to-file

cargo run --example to-file-debug-plus-release
cargo run --example to-file-debug-plus-release --release
```
```sh
cd ./examples/nolog-no-dep-to-stdout/
cargo run
```
```sh
cd ./examples/nolog-as-dep-to-stdout/
cargo run
```

If you want to log to a file and are going to use `nolog` as a dependency,
then use [nolog-plain](https://crates.io/crates/nolog-plain) crate (а non-colored version of nolog).
Characters used for text coloring will not be saved to the file.

Example using `nolog-plain` crate:

```sh
cd ./examples/nolog-plain-as-dep-to-file/
cargo run 
```
```sh
cd ./examples/nolog-plain-as-dep-to-file-timestamp-chrono/
cargo run 
```

## Using nolog

Create a `logger.rs` with the following content (this is the complete
library code):

```rust
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
```

`main.rs` could be like this:

```rust
#[macro_use]
mod logger;
//  ^^^^^^
//  Must go higher in the list than the modules
//  in which it will be used.
//  Macro import in modules below is not needed.
mod other;

fn main() {
    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    /* Output:
    TRCE: text 42,42,42 [examples/to-stderr.rs 55:5]
    DEBG: text 42,42,24 [examples/to-stderr.rs 56:5]
    INFO: text 42,24,42 [examples/to-stderr.rs 57:5]
    WARN: text 42,a,422 [examples/to-stderr.rs 58:5]
    ERRO: text 42,42,42 [examples/to-stderr.rs 59:5]
    CRIT: text 42,42,42 [examples/to-stderr.rs 60:5]
    */
}
```
Now you have a logger and no new dependencies.

## Redirect output

To redirect output to `stderr` you need to add a single 'e' character
to the `writelog` macro so that instead of `println!()`
it becomes `eprintln!()`. [See example](https://github.com/vglinka/nolog/blob/main/examples/to-stderr.rs).

Write log to file: [See example](https://github.com/vglinka/nolog/blob/main/examples/to-file.rs).

## Using nolog as a dependency

Although this logger was created to be used directly in small projects,
you can also use it as a dependency.

In `Cargo.toml`:

```toml
nolog = "0.2.3"
```

In `main.rs`:
```rust
use nolog::*;
```

OR

```rust
use nolog::{crit, debug, error, info, trace, warn, writelog};
```

OR

```rust
#[macro_use]
pub extern crate nolog;
```

The last option with `extern` is the preferred option. You don't have
to import macros in every module, they will work without imports.
[See example](https://github.com/vglinka/nolog/blob/main/examples/nolog-as-dep-to-stdout/src/main.rs).

## nolog-plain (а non-colored version of nolog)

If you want to log to a file and are going to use `nolog` as a dependency,
then use [nolog-plain](https://crates.io/crates/nolog-plain) crate.
Characters used for text coloring will not be saved to the file.

## How to add Timestamp
For example, we will use [chrono crate](https://docs.rs/chrono/0.4.19/chrono/).

```
-- In "log.txt" --
[2022-07-10 06:49:33.646361181 UTC] TRCE: text 42,42,42 [src/main.rs 22:5]
[2022-07-10 06:49:33.646393648 UTC] DEBG: text 42,42,24 [src/main.rs 23:5]
[2022-07-10 06:49:33.646405179 UTC] INFO: text 42,24,42 [src/main.rs 24:5]
[2022-07-10 06:49:33.646415125 UTC] WARN: text 42,a,422 [src/main.rs 25:5]
[2022-07-10 06:49:33.646424722 UTC] ERRO: text 42,42,42 [src/main.rs 26:5]
[2022-07-10 06:49:33.646434216 UTC] CRIT: text 42,42,42 [src/main.rs 27:5]
```
[See example](https://github.com/vglinka/nolog/blob/main/examples/nolog-plain-as-dep-to-file-timestamp-chrono/src/logger.rs).

## If you don't need to disable logging on a release build

This will make the code even shorter.
[See example](https://github.com/vglinka/nolog/blob/main/examples/to-file-debug-plus-release.rs).

## Changelog

- **0.2.3** – Changes in `README.md` and examples.
- **0.2.2** – Changes in `Cargo.toml` (`categories` field has been added).
- **0.2.1** – Changes in `README.md`
- **0.2.0** – Macro invocation `format!()` have been replaced with
  invocation `format_args!()` because `format_args!()` avoids heap
  allocations. All examples have been updated. The use of `format_args!()`
  has affected the code for writing to a file. Now `std::fmt::Arguments`
  is used instead of `String`.
  [See example](https://github.com/vglinka/nolog/blob/main/examples/nolog-plain-as-dep-to-file/src/logger.rs).
- **0.1.3** – Changes in `README.md` and examples.
- **0.1.2** – Changes in `README.md` and examples.
- **0.1.1** – Changes in `README.md`
