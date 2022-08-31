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



