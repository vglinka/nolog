// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

#[macro_export] macro_rules!
glob_access { ( $name:expr, $var:ident, $code:block ) => { 
        $name.with(
            |$var| {
                std::cell::RefMut::map(
                    $var.borrow_mut(), 
                    |$var| {
                        $code
                    }
                );
            } 
        );     
    } 
}

#[macro_export] macro_rules!
glob_replace { ( $name:expr, $code:expr ) => { 
        let _ = $name.with(|x| x.replace($code));
    }
}



