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
level_helper { 
    // On:  [#], [x], [v], [+], [true], [bool_var], [on]
    // Off: [ ], [_], [-], [false], [bool_var], [off]
    //
    //
    //        +----------+                                      +------------------------+    
    //        | one line |                                      | lines separated by `;` |
    //        +----------+                                      +------------------------+  
    //          v      v                                                v        v
    // +----------+  +----------------------+     +------------------------+  +------------------------------------+ 
    // | one line |  | one line (key-value) |     | lines separated by `;` |  | lines (key-value) separated by `;` |
    // +----------+  +----------------------+     +------------------------+  +------------------------------------+ 
    //     |  |  |           |   |    |                       |  |    |                      |  |    |
    //     v  |  v           v   |    v                       v  |    v                      v  |    v
    // +----+ | +-----+   +----+ | +-----+                +----+ | +-----+               +----+ | +-----+
    // | On | | | Off |   | On | | | Off |                | On | | | Off |               | On | | | Off |
    // +----+ | +-----+   +----+ | +-----+                +----+ | +-----+               +----+ | +-----+
    //        v                  v                               v                              V
    // +------------+      +------------+                  +------------+                 +------------+
    // | var (bool) |      | var (bool) |                  | var (bool) |                 | var (bool) |
    // +------------+      +------------+                  +------------+                 +------------+
    //
    //
    //
    //
    // Off, one line
    ( [$level:tt] [ ];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {};
    ( [$level:tt] [_];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {};
    ( [$level:tt] [-];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {};
    ( [$level:tt] [off]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {};
    // On, one line
    ( [$level:tt] [#];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*);
    };
    ( [$level:tt] [x];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*);
    };
    ( [$level:tt] [v];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*);
    };
    ( [$level:tt] [+];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*);
    };
    ( [$level:tt] [on]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*);
    };
    //
    // Off, one line (key-value)
    ( [$level:tt] [ ];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => {};
    ( [$level:tt] [_];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => {};
    ( [$level:tt] [-];   $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => {};
    ( [$level:tt] [off]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => {};
    // On, one line (key-value)
    ( [$level:tt] [#];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*);
    };
    ( [$level:tt] [x];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*);
    };
    ( [$level:tt] [v];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*);
    };
    ( [$level:tt] [+];  $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*);
    };
    ( [$level:tt] [on]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*);
    };
    //
    //
    // Off, lines separated by `;`
    ( [$level:tt] [ ];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {};
    ( [$level:tt] [_];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {};
    ( [$level:tt] [-];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {};
    ( [$level:tt] [off]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {};
    // On, lines separated by `;`
    ( [$level:tt] [#];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    ( [$level:tt] [x];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    ( [$level:tt] [v];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    ( [$level:tt] [+];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    ( [$level:tt] [on]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    //
    // Off, lines (key-value) separated by `;`
    ( [$level:tt] [ ];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {};
    ( [$level:tt] [_];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {};
    ( [$level:tt] [-];   $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {};
    ( [$level:tt] [off]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {};
    // On, lines (key-value) separated by `;`
    ( [$level:tt] [#];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    ( [$level:tt] [x];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    ( [$level:tt] [v];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    ( [$level:tt] [+];  $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    ( [$level:tt] [on]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    //
    // var (bool), one line
    ( [$level:tt] [$var:tt]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        //  debug!([(is_log_enabled_fn())]; "msg");
        //          ^                   ^
        // fn doesn't work without parentheses
        #[allow(unused_parens)]
        // `($var)` doesn't help
        if $var { level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); }
    };
    //
    // var (bool), one line (key-value)
    ( [$level:tt] [$var:tt]; $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        #[allow(unused_parens)]
        if $var { level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); }
    };
    //
    // var (bool), lines separated by `;`
    ( [$level:tt] [$var:tt]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        #[allow(unused_parens)]
        if $var { $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )* }
    };
    //
    // var (bool), lines (key-value) separated by `;`
    ( [$level:tt] [$var:tt]; $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        #[allow(unused_parens)]
        if $var { $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )* }
    };
    //
    //
    // lines separated by `;` (without on/off)
    ( [$level:tt] $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),*;)* ) => {
        // `one line` invocation for each line
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($msg),*); )*
    };
    // lines (key-value) separated by `;` (without on/off)
    ( [$level:tt] $($(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),*;)* ) => {
        // `one line (key-value)` invocation for each line
        $( level_helper!([$level] $(->[$indent $(,$linebefore $(,$lineafter)?)?])? $($key),* => $(->[$indent_v])? $($value),*); )*
    };
    //
    //
    // one line
    ( [$level:tt] $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($msg:expr),* ) => {
        $($(newline!($linebefore);)?)?
        msg_render!($level, indent!($($indent)?), $($msg),*);
        $($($(newline!($lineafter);)?)?)?
    };
    // one line (key-value)
    ( [$level:tt] $(->[$indent:tt $(,$linebefore:tt $(,$lineafter:tt)?)?])? $($key:expr),* => $(->[$indent_v:tt])? $($value:expr),* ) => { 
        $($(newline!($linebefore);)?)?
        msg_render!($level, indent_kv!($($indent)?), $($key),* => indent!($($indent_v)?), $($value),*);
        $($($(newline!($lineafter);)?)?)?
    };
}

#[macro_export]
#[cfg(feature = "trace")] macro_rules!
trace_inner { ( $($a:tt)* ) => { level_helper!([[trace]] $($a)*); } }

#[macro_export]
#[cfg(feature = "debug")] macro_rules!
debug_inner { ( $($a:tt)* ) => { level_helper!([[debug]] $($a)*); } }

#[macro_export]
#[cfg(feature = "info")] macro_rules!
info_inner { ( $($a:tt)* )  => { level_helper!([[info]] $($a)*); } }

#[macro_export]
#[cfg(feature = "warn")] macro_rules!
warn_inner { ( $($a:tt)* )  => { level_helper!([[warn]] $($a)*); } }

#[macro_export]
#[cfg(feature = "error")] macro_rules!
error_inner { ( $($a:tt)* ) => { level_helper!([[error]] $($a)*); } }

#[macro_export]
#[cfg(feature = "crit")] macro_rules!
crit_inner { ( $($a:tt)* )  => { level_helper!([[crit]] $($a)*); } }

