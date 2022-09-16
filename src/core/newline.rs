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
#[cfg(feature = "newline_ignore")] macro_rules!
newline { 
    (     ) => {};
    ($a:tt) => {};
}

#[macro_export] 
#[cfg(not(feature = "newline_ignore"))] macro_rules!
newline { 
    ( )  => { writelog!([[none]], [[newline]], "") };
    (_)  => { };
    (0)  => { };
    (1)  => { writelog!([[none]], [[newline]], "") };
    (2)  => { writelog!([[none]], [[newline]], "\n") };
    (3)  => { writelog!([[none]], [[newline]], "\n\n") };
    (4)  => { writelog!([[none]], [[newline]], "\n\n\n") };
    (5)  => { writelog!([[none]], [[newline]], "\n\n\n\n") };
    (6)  => { writelog!([[none]], [[newline]], "\n\n\n\n\n") };
    (7)  => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n") };
    (8)  => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n") };
    (9)  => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n") };
    (10) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n") };
    (11) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n") };
    (12) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n") };
    (13) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n") };
    (14) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (15) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (16) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (17) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (18) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (19) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (20) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (21) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (22) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (23) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (24) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (25) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (26) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (27) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (28) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (29) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    (30) => { writelog!([[none]], [[newline]], "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n") };
    // This arm is used in two cases:
    // 1. When i > 30
    // 2. If a variable was used to set the value: `newline(my_var)`.
    //    Therefore `match` was used to avoid heap allocation (String)
    //    when it is not necessary.
    //
    // P.S. `match $i { n @ 1..=30 => newline!(n), }` doesn't work here.
    // `newline!(n)` is the same as `newline(my_var)`, It will cause
    // an infinite recursion.
    ($i:expr) => {       
        if $i > 0 {
            if $i <= 30 {
                match $i {
                    1  => newline!(1),
                    2  => newline!(2),
                    3  => newline!(3),
                    4  => newline!(4),
                    5  => newline!(5),
                    6  => newline!(6),
                    7  => newline!(7),
                    8  => newline!(8),
                    9  => newline!(9),
                    10 => newline!(10),
                    11 => newline!(11),
                    12 => newline!(12),
                    13 => newline!(13),
                    14 => newline!(14),
                    15 => newline!(15),
                    16 => newline!(16),
                    17 => newline!(17),
                    18 => newline!(18),
                    19 => newline!(19),
                    20 => newline!(20),
                    21 => newline!(21),
                    22 => newline!(22),
                    23 => newline!(23),
                    24 => newline!(24),
                    25 => newline!(25),
                    26 => newline!(26),
                    27 => newline!(27),
                    28 => newline!(28),
                    29 => newline!(29),
                    30 => newline!(30),          
                    _  => unreachable!(),
                }
            } else {
                let mut line = String::from("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                for _ in 0..($i - 30) { line.push_str("\n") };
                writelog!([[none]], [[newline]], line);
            }
        }        
    };
}

