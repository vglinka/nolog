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
#[cfg(not(feature = "custom_leading"))] macro_rules!
custom_leading { 
    ( $level:tt, $indent:expr, $($msg:expr),* ) => { "" };
    ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => { "" };
}

#[macro_export]
#[cfg(not(feature = "custom_trailing"))] macro_rules!
custom_trailing {
    ( $level:tt, $indent:expr, $($msg:expr),* ) => { "" };
    ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => { "" };
}

#[macro_export]
#[cfg(not(feature = "custom_before_msg"))] macro_rules!
custom_before_msg { 
    ( $level:tt, $indent:expr, $($msg:expr),* ) => { "" };
    ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => { "" };
}

#[macro_export]
#[cfg(not(feature = "custom_after_msg"))] macro_rules!
custom_after_msg {
    ( $level:tt, $indent:expr, $($msg:expr),* ) => { "" };
    ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => { "" };
}
