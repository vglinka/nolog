// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(
    all(
        not(feature = "sep_colon"),
        not(feature = "sep_hide"),
        not(feature = "sep_space"),
        not(feature = "custom_sep"),
))] macro_rules!
sep { () => { "⧽ " }; }

#[macro_export]
#[cfg(feature = "sep_colon")] macro_rules!
sep { () => { ": " }; }

#[macro_export]
#[cfg(feature = "sep_space")] macro_rules!
sep { () => { " " }; }

#[macro_export]
#[cfg(feature = "sep_hide")] macro_rules!
sep { () => { "" }; }

#[macro_export]
#[cfg(feature = "show_lvl_header_kv")] macro_rules!
sep_kv { () => { sep!() }; }

#[macro_export]
#[cfg(not(feature = "show_lvl_header_kv"))] macro_rules!
sep_kv { () => { "" } }

