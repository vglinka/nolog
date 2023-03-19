// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export] macro_rules!
logon { () => { glob_replace!($crate::log_enabled::LOG_ENABLED, true) } }

#[macro_export] macro_rules!
logoff { () => { glob_replace!($crate::log_enabled::LOG_ENABLED, false) } }

#[macro_export] macro_rules!
logonly {
    // Off
    ( [ ];   $($a:tt)* ) => { $($a)*; };
    ( [_];   $($a:tt)* ) => { $($a)*; };
    ( [-];   $($a:tt)* ) => { $($a)*; };
    ( [off]; $($a:tt)* ) => { $($a)*; };
    // On
    ( [x];  $($a:tt)* ) => { logonly!($($a)*); };
    ( [v];  $($a:tt)* ) => { logonly!($($a)*); };
    ( [+];  $($a:tt)* ) => { logonly!($($a)*); };
    ( [#];  $($a:tt)* ) => { logonly!($($a)*); };
    ( [on]; $($a:tt)* ) => { logonly!($($a)*); };
    // bool variable
    ( [$var:tt]; $($a:tt)* ) => { 
        if $var { logon!(); }
        $($a)*;
        if $var { logoff!(); }
    };
    // Without `[]`
    ( $($a:tt)* ) => {
        logon!();
        $($a)*;
        logoff!();
    };
}

#[macro_export] macro_rules!
logonly_helper {
    ( $($msg:tt)* ) => {
        glob_access!(
            $crate::log_enabled::LOG_ENABLED,
            log_enabled,
            { 
                match log_enabled {
                    true => { $($msg)*; /* print msg*/ },
                    false => (),
                }
                log_enabled
            }
        );
    };
}
