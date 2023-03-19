// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(
    all(
        not(feature = "logcatch"),
        not(feature = "custom_writelog"),
))] macro_rules!
writelog { ( [$level:tt], [$msg_type:tt], $msg:expr ) => { writelog_inner!($msg) } }

#[macro_export]
#[cfg(
    all(
        not(feature = "tofile"),
        not(feature = "custom_writelog_inner"),
))] macro_rules!
writelog_inner { ( $msg:expr ) => { eprintln!("{}", $msg) } }
