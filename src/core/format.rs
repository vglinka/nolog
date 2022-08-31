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

#[macro_export] macro_rules!
indents_format {
    ( $indent:expr ) => { format_args!("{}{}", indent_base!(), $indent) };
}

#[macro_export] macro_rules!
lvlheader_format {
    ( $level:tt ) => { format_args!("{}{}{}", color!($level), lvlheader!($level), color!([rm])) };
}

#[macro_export] macro_rules!
sep_format {
    ( $level:tt ) => { format_args!("{}{}{}{}", color!($level), color!([sep]), sep!(), color!([rm])) };
}

#[macro_export] macro_rules!
sep_kv_format {
    ( $level:tt ) => { format_args!("{}{}{}{}", color!($level), color!([sep]), sep_kv!(), color!([rm])) };
}

#[macro_export] macro_rules!
msg_format {
    ( $($msg:expr),* ) => {
        format_args!("{}{}{}", color!([msg]), format_args!($($msg),*), color!([rm])) };
}

#[macro_export] macro_rules!
lvlheader_kv_format {
    ( $level:tt ) => { format_args!("{}{}{}", color!($level), lvlheader_kv!($level), color!([rm])) };
}

#[macro_export] macro_rules!
key_format {
    ( $level:tt => $($key:expr),* ) => {
        format_args!("{}{}{}{}",
            color!($level), color!([key]), format_args!($($key),*), color!([rm])) };
}

#[macro_export] macro_rules!
sep_key {
    ( $level:tt ) => {
        format_args!("{}{}{}{}",
            color!($level), color!([sep]), sep!(), color!([rm])) };
}

#[macro_export] macro_rules!
value_format {
    ( $($value:expr),* ) => {
        format_args!("{}{}{}", color!([value]), format_args!($($value),*), color!([rm])) };
}

#[macro_export]
#[cfg(
    all(
        not(feature = "location_hide"),
        not(feature = "location_style_classic"),
        not(feature = "custom_location_style"),
))] macro_rules!
location_format {
    () => { format_args!(" {}{}{}{}{}{}{}{}{} {}{}{}", 
        color!([sep2]), "[", color!([rm]),
        color!([line]), line!(), color!([rm]),
        color!([sep3]), "]", color!([rm]),
        color!([from]), file!(), color!([rm]))
    };
}

#[macro_export]
#[cfg(
    all(
        not(feature = "location_hide"),
        not(feature = "custom_location_style"),
        feature = "location_style_classic",
))] macro_rules!
location_format {
    () => { format_args!(" {}[{} {}:{}]{}", color!([from]), file!(), line!(), column!(), color!([rm])) };
}

#[macro_export]
#[cfg(
    all(
        feature = "location_hide",
        not(feature = "custom_location_style"),
        not(feature = "location_style_classic"),
))] macro_rules!
location_format {
    () => { "" };
}
