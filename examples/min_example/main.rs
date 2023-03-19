// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_use]
extern crate nolog;

// use `cargo run --features trace`

fn main() {
    trace!("line_count: {}", 42);
    debug!("line_count: {}", 42);
    info!("line_count: {}", 42);
    warn!("line_count: {}", 42);
    error!("line_count: {}", 42);
    crit!("line_count: {}", 42);
}

