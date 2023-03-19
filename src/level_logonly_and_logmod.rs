// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(feature = "trace")] macro_rules!
trace { ( $($a:tt)* ) => { logonly_helper!(logmod_helper!(trace_inner!($($a)*))); } }

#[macro_export]
#[cfg(feature = "debug")] macro_rules!
debug { ( $($a:tt)* ) => { logonly_helper!(logmod_helper!(debug_inner!($($a)*))); } }

#[macro_export]
#[cfg(feature = "info")] macro_rules!
info { ( $($a:tt)* )  => { logonly_helper!(logmod_helper!(info_inner!($($a)*))); } }

#[macro_export]
#[cfg(feature = "warn")] macro_rules!
warn { ( $($a:tt)* )  => { logonly_helper!(logmod_helper!(warn_inner!($($a)*))); } }

#[macro_export]
#[cfg(feature = "error")] macro_rules!
error { ( $($a:tt)* ) => { logonly_helper!(logmod_helper!(error_inner!($($a)*))); } }

#[macro_export]
#[cfg(feature = "crit")] macro_rules!
crit { ( $($a:tt)* )  => { logonly_helper!(logmod_helper!(crit_inner!($($a)*))); } }
