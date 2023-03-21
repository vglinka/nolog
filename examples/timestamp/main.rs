// This example is licensed under the CC0 license.
// https://creativecommons.org/share-your-work/public-domain/cc0
//
// This means that you can use the code from this example in your projects
// without any restrictions or attribution.

#[macro_use]
extern crate nolog;

// use `cargo run --features trace`

#[macro_use] 
pub mod logger_setup {
    #[macro_export]
    #[cfg(feature = "custom_leading")] macro_rules!
    //               ^^^^^^^^^^^^^^
    custom_leading { 
        // usual
        ( $level:tt, $indent:expr, $($msg:expr),* ) => {
            format_args!("[{}] ", chrono::Utc::now())
            
        };
        // key-value
        ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => {
            format_args!("[{}] ", chrono::Local::now())
        };
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

fn main() {
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();   
}
