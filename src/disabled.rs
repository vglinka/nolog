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
#[cfg(
    any(
        all(debug_assertions, not(feature = "trace")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "trace"))
))] macro_rules!
trace { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "debug")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "debug"))
))] macro_rules!
debug { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "info")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "info"))
))] macro_rules!
info { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "warn")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "warn"))
))] macro_rules!
warn  { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "error")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "error"))
))] macro_rules!
error { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "crit")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "crit"))
))] macro_rules!
crit { ( $($a:tt)* ) => () }


#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "tofile")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "tofile"))
))] macro_rules!
logfile { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "tofile")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "tofile"))
))] macro_rules!
logflush { () => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "glob")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "glob"))
))] macro_rules!
glob_access { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "glob")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "glob"))
))] macro_rules!
glob_replace { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(all(not(debug_assertions), not(feature = "release")))] macro_rules!
newline { 
    ()  => ();
    ( $($a:tt)* ) => ()
}

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "logonly")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "logonly"))
))] macro_rules!
logon { ()  => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "logonly")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "logonly"))
))] macro_rules!
logoff { ()  => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "logonly")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "logonly"))
))] macro_rules!
logonly {
    // On
    ( [#]; $($a:tt)* ) => { $($a)* };
    ( [x]; $($a:tt)* ) => { $($a)* };
    ( [v]; $($a:tt)* ) => { $($a)* };
    ( [+]; $($a:tt)* ) => { $($a)* };
    ( [on]; $($a:tt)* ) => { $($a)* };
    // Off
    ( [ ]; $($a:tt)* ) => { $($a)* };
    ( [_]; $($a:tt)* ) => { $($a)* };
    ( [-]; $($a:tt)* ) => { $($a)* };
    ( [off]; $($a:tt)* ) => { $($a)* };
    // bool variable
    ( [$var:tt]; $($a:tt)* ) => { $($a)* };
    // Without `[]`
    ( $($a:tt)* ) => { $($a)* };
}

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "logmod")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "logmod"))
))] macro_rules!
logmod { ( $($a:tt)* ) => () }

#[macro_export]
#[cfg(
    any(
        all(debug_assertions, not(feature = "logcatch")),
        all(not(debug_assertions), not(feature = "release")),
        all(not(debug_assertions), not(feature = "logcatch"))
))] macro_rules!
logcatch { ( $($a:tt)* ) => () }





