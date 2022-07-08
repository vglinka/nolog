mod logger;

use crate::logger::*;
//         ^^^^^^^^^
// It must be imported in each module in which we will use logging.

use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

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
    TRCE: text 42,42,42 [src/main.rs 29:5]
    DEBG: text 42,42,24 [src/main.rs 30:5]
    INFO: text 42,24,42 [src/main.rs 31:5]
    WARN: text 42,a,422 [src/main.rs 32:5]
    ERRO: text 42,42,42 [src/main.rs 33:5]
    CRIT: text 42,42,42 [src/main.rs 34:5]
    */

    Ok(())
}
