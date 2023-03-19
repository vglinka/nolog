// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(
    not(feature = "custom_lvl_headers")
)] macro_rules!
lvlheader { 
    ( [trace] ) => { "TRCE" };
    ( [debug] ) => { "DEBG" };
    ( [info]  ) => { "INFO" };
    ( [warn]  ) => { "WARN" };
    ( [error] ) => { "ERRO" };
    ( [crit]  ) => { "CRIT" };
}

#[macro_export]
#[cfg(
    all(
        not(feature = "custom_lvl_headers"),
        feature = "show_lvl_header_kv"
))] macro_rules!
lvlheader_kv { 
    ( [trace] ) => { "TRCE" };
    ( [debug] ) => { "DEBG" };
    ( [info]  ) => { "INFO" };
    ( [warn]  ) => { "WARN" };
    ( [error] ) => { "ERRO" };
    ( [crit]  ) => { "CRIT" };
}

#[macro_export]
#[cfg(
    all(
        not(feature = "custom_lvl_headers"),
        not(feature = "show_lvl_header_kv")
))] macro_rules!
lvlheader_kv { ( [$a:tt] ) => { "" } }
