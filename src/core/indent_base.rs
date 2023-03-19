// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export]
#[cfg(  
    all(
        not(feature = "indent_ignore_all"),
        not(feature = "indent_base_zero"),
        not(feature = "indent_base_one"),
        not(feature = "indent_base_two"),
        not(feature = "indent_base_three"),
        not(feature = "indent_base_four"),
        not(feature = "indent_base_five"),
        not(feature = "indent_base_seven"),
        not(feature = "indent_base_eight"),
        not(feature = "indent_base_nine"),
        not(feature = "indent_base_ten"),
))] macro_rules!
indent_base { () => { indent!(6) } } // Default indent_base = 6 indents

#[macro_export] 
#[cfg(
    any(
        feature = "indent_ignore_all",
        all(
            feature = "indent_base_zero",
            not(feature = "indent_ignore_all")
)))] macro_rules!
indent_base { () => { "" } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_one",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(1) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_two",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(2) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_three",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(3) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_four",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(4) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_five",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(5) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_six",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(6) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_seven",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(7) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_eight",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(8) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_nine",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(9) } }

#[macro_export] 
#[cfg(
    all(
        feature = "indent_base_ten",
        not(feature = "indent_ignore_all")
))] macro_rules!
indent_base { () => { indent!(10) } }



