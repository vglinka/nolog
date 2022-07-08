mod other;

#[macro_use]
extern crate nolog;

#[allow(unused_variables)]
fn main() {
    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    println!("");
    other::from_other();

    /* Outout:
    TRCE: text 42,42,42 [src/main.rs 9:5]
    DEBG: text 42,42,24 [src/main.rs 10:5]
    INFO: text 42,24,42 [src/main.rs 11:5]
    WARN: text 42,a,422 [src/main.rs 12:5]
    ERRO: text 42,42,42 [src/main.rs 13:5]
    CRIT: text 42,42,42 [src/main.rs 14:5]

    CRIT: From other mod! [src/other.rs 2:5]
    */
}
