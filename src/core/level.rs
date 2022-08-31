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
#[cfg(feature = "trace")] macro_rules!
trace { ( $($a:tt)* ) => { trace_inner!($($a)*); } }

#[macro_export]
#[cfg(feature = "debug")] macro_rules!
debug { ( $($a:tt)* ) => { debug_inner!($($a)*); } }

#[macro_export]
#[cfg(feature = "info")] macro_rules!
info { ( $($a:tt)* ) => { info_inner!($($a)*); } }

#[macro_export]
#[cfg(feature = "warn")] macro_rules!
warn { ( $($a:tt)* ) => { warn_inner!($($a)*); } }

#[macro_export]
#[cfg(feature = "error")] macro_rules!
error { ( $($a:tt)* ) => { error_inner!($($a)*); } }

#[macro_export]
#[cfg(feature = "crit")] macro_rules!
crit { ( $($a:tt)* ) => { crit_inner!($($a)*); } }
