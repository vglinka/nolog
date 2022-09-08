// Copyright (c) 2022 Vadim Glinka
//
// See the COPYRIGHT file at the top-level directory of this distribution
// and at https://github.com/vglinka/nolog/blob/main/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
