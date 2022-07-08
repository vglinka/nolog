/*
When using extern with `#[macro_use]`, importing macros is not required.
When writing the log to a file, you still have to import the structures,
so using extern will not change much.
*/
pub fn from_other() {
    crit!("From other mod!")
}
