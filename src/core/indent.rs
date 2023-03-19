// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export] 
#[cfg(feature = "indent_ignore_all")] macro_rules!
indent {
    (            ) => { "" };
    ( _          ) => { "" };
    ( $indent:tt ) => { "" };
}

#[macro_export] 
#[cfg(feature = "indent_ignore_all")] macro_rules!
indent_kv { 
    (            ) => { "" };
    ( _          ) => { "" };
    ( $indent:tt ) => { "" };
}

#[macro_export]
#[cfg(    
    all(
        not(feature = "indent_ignore_all"),
        not(feature = "indent_kv_default_zero"),
        not(feature = "indent_kv_default_one"),
        not(feature = "indent_kv_default_two"),
        not(feature = "indent_kv_default_three"),
        not(feature = "indent_kv_default_four"),
        not(feature = "indent_kv_default_five"),        
        not(feature = "indent_kv_default_seven"),
        not(feature = "indent_kv_default_eight"),
        not(feature = "indent_kv_default_nine"),
        not(feature = "indent_kv_default_ten"),
))] macro_rules!
indent_kv { 
    (            ) => { indent!(6)       }; // default 6 indents for key-value
    ( _          ) => { indent!(6)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_zero")] macro_rules!
indent_kv { 
    (            ) => { indent!(0)       };
    ( _          ) => { indent!(0)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_one")] macro_rules!
indent_kv { 
    (            ) => { indent!(1)       };
    ( _          ) => { indent!(1)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_two")] macro_rules!
indent_kv { 
    (            ) => { indent!(2)       };
    ( _          ) => { indent!(2)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_three")] macro_rules!
indent_kv { 
    (            ) => { indent!(3)       };
    ( _          ) => { indent!(3)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_four")] macro_rules!
indent_kv { 
    (            ) => { indent!(4)       };
    ( _          ) => { indent!(4)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_five")] macro_rules!
indent_kv { 
    (            ) => { indent!(5)       };
    ( _          ) => { indent!(5)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_seven")] macro_rules!
indent_kv { 
    (            ) => { indent!(7)       };
    ( _          ) => { indent!(7)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_eight")] macro_rules!
indent_kv { 
    (            ) => { indent!(8)       };
    ( _          ) => { indent!(8)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_nine")] macro_rules!
indent_kv { 
    (            ) => { indent!(9)       };
    ( _          ) => { indent!(9)       };
    ( $indent:tt ) => { indent!($indent) };
}

#[macro_export] 
#[cfg(feature = "indent_kv_default_ten")] macro_rules!
indent_kv { 
    (            ) => { indent!(10)       };
    ( _          ) => { indent!(10)       };
    ( $indent:tt ) => { indent!($indent)  };
}


#[macro_export] 
#[cfg(not(feature = "indent_ignore_all"))] macro_rules!
indent { 
    ( )  => { "" };  
    (_)  => { "" }; 
    (0)  => { "" };
    (1)  => { " " };
    (2)  => { "  " };
    (3)  => { "   " };
    (4)  => { "    " };
    (5)  => { "     " };
    (6)  => { "      " };
    (7)  => { "       " };
    (8)  => { "        " };
    (9)  => { "         " };
    (10) => { "          " };
    (11) => { "           " };
    (12) => { "            " };
    (13) => { "             " };
    (14) => { "              " };
    (15) => { "               " };
    (16) => { "                " };
    (17) => { "                 " };
    (18) => { "                  " };
    (19) => { "                   " };
    (20) => { "                    " };
    (21) => { "                     " };
    (22) => { "                      " };
    (23) => { "                       " };
    (24) => { "                        " };
    (25) => { "                         " };
    (26) => { "                          " };
    (27) => { "                           " };
    (28) => { "                            " };
    (29) => { "                             " };
    (30) => { "                              " };
    // This arm is used in two cases:
    // 1. When i > 30
    // 2. If a variable was used to set the value: `trace!(->[my_var] "")`.
    //
    // P.S. `match $i { n @ 1..=30 => indent!(n), }` doesn't work here.
    // `indent!(n)` is the same as `indent(my_var)`, It will cause
    // an infinite recursion.
    ( $i:expr ) => {
        {
            if $i > 0 && $i <= 30 {
                match $i {
                        1  => indent!(1),
                        2  => indent!(2),
                        3  => indent!(3),
                        4  => indent!(4),
                        5  => indent!(5),
                        6  => indent!(6),
                        7  => indent!(7),
                        8  => indent!(8),
                        9  => indent!(9),
                        10 => indent!(10),
                        11 => indent!(11),
                        12 => indent!(12),
                        13 => indent!(13),
                        14 => indent!(14),
                        15 => indent!(15),
                        16 => indent!(16),
                        17 => indent!(17),
                        18 => indent!(18),
                        19 => indent!(19),
                        20 => indent!(20),
                        21 => indent!(21),
                        22 => indent!(22),
                        23 => indent!(23),
                        24 => indent!(24),
                        25 => indent!(25),
                        26 => indent!(26),
                        27 => indent!(27),
                        28 => indent!(28),
                        29 => indent!(29),
                        30 => indent!(30),          
                        _  => unreachable!(),
                    }.into()
            } else if $i > 30 {
                let mut indent = String::from(indent!(30));
                for _ in 0..($i - 30) { indent.push_str(indent!(1)) };
                indent
            } else {
                "".into()
            }
        }
    };
}

