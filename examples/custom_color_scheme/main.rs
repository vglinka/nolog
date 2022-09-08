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
    #[cfg(feature = "custom_colors")] macro_rules!
    //               ^^^^^^^^^^^^^
    color {
        ( [trace] ) => { "\x1B[34m"                    };
        ( [debug] ) => { "\x1B[36m"                    };
        ( [info]  ) => { "\x1B[32m"                    };
        ( [warn]  ) => { "\x1B[33m"                    };
        ( [error] ) => { "\x1B[31m"                    };
        ( [crit]  ) => { "\x1B[35m"                    };
        ( [sep]   ) => { "\x1B[1m\x1B[2m"              }; // +bold +dim
        ( [msg]   ) => { ""                            }; // default term font color
        ( [from]  ) => { "\x1B[90m\x1B[3m"             }; // `[src/main.rs 101:5]` in `location_style_classic`
        ( [sep2]  ) => { "\x1B[90m\x1B[2m"             }; // sep2 in default style
        ( [sep3]  ) => { "\x1B[90m\x1B[2m"             }; // sep3 in default style
        ( [line]  ) => { "\x1B[38;5;67m\x1B[1m\x1B[2m" }; // line number in default style
        ( [key]   ) => { "\x1B[3m\x1B[1m"              }; // +italic +bold 
        ( [value] ) => { "\x1B[3m"                     }; // +italic
        ( [rm]    ) => { "\x1B[0m"                     }; // remove previous colors
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

fn main(){
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();   
}
