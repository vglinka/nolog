// This example is licensed under the CC0 license.
// https://creativecommons.org/share-your-work/public-domain/cc0
//
// This means that you can use the code from this example in your projects
// without any restrictions or attribution.

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

