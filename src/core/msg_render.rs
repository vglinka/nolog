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

#[macro_export]
#[cfg(not(feature = "custom_msg_render"))] macro_rules!
msg_render {
    ( $level:tt, $indent:expr, $($msg:expr),* ) => {
        writelog!(
            [$level],
            [[usual]],
            format_args!(
                "{}{}{}{}{}{}{}{}{}",
                indents_format!($indent),
                custom_leading!($level, $indent, $($msg),*),
                lvlheader_format!($level),
                sep_format!($level),
                custom_before_msg!($level, $indent, $($msg),*),
                msg_format!($($msg),*),
                custom_after_msg!($level, $indent, $($msg),*),
                location_format!(),
                custom_trailing!($level, $indent, $($msg),*),
            )
        )
    };
    ( $level:tt, $indent:expr, $($key:expr),* => $indent_v:expr, $($value:expr),* ) => {
        writelog!(
            [$level],
            [[key_value]],
            format_args!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                indents_format!($indent),
                custom_leading!($level, $indent, $($key),* => $($value),*),
                lvlheader_kv_format!($level),
                sep_kv_format!($level),
                custom_before_msg!($level, $indent, $($key),* => $($value),*),
                key_format!($level => $($key),*),
                sep_key!($level),
                $indent_v,
                value_format!($($value),*),
                custom_after_msg!($level, $indent, $($key),* => $($value),*),
                location_format!(),
                custom_trailing!($level, $indent, $($key),* => $($value),*),
            )
        )
    };
}
