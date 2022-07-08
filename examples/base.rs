/*
In Cargo.toml:
  ```
  nolog = "0.2.3"
  ```

In main.rs:
  ```
  use nolog::*;
  ```

OR

  ```
  use nolog::{crit, debug, error, info, trace, warn, writelog};
  ```

OR

  ```
  #[macro_use]
  pub extern crate nolog;
  ```
  
The last option with `extern` is the preferred option. You don't have
to import macros in every module, they will work without imports.
See example: ./nolog-as-dep-to-stdout/src/main.rs
*/

use nolog::*;

#[allow(unused_variables)]
fn main() {
    let a = 42;
    trace!("text {a},{a},{a}");
    debug!("text {a},{},{}", a, 24);
    info!("text {},{},{}", a, 24, "42");
    warn!("text {a},{},{}", 'a', "422");
    error!("text {a},{a},{}", a);
    crit!("text {a},{a},{a}");

    /* Outout:
    TRCE: text 42,42,42 [examples/to-stderr.rs 55:5]
    DEBG: text 42,42,24 [examples/to-stderr.rs 56:5]
    INFO: text 42,24,42 [examples/to-stderr.rs 57:5]
    WARN: text 42,a,422 [examples/to-stderr.rs 58:5]
    ERRO: text 42,42,42 [examples/to-stderr.rs 59:5]
    CRIT: text 42,42,42 [examples/to-stderr.rs 60:5]
    */
}
