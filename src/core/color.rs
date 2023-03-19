// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(
    all(
        not(feature = "custom_colors"),
        not(feature = "plain")
))] macro_rules!
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

#[macro_export]
#[cfg(
    all(
        not(feature = "custom_colors"),
        feature = "plain"
))] macro_rules!
color { ( [$a:tt] ) => { "" } }
