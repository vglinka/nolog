// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

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
