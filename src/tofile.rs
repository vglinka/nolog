// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

pub use std::cell::{RefCell, RefMut};
pub use std::io::{BufWriter, Write};
pub use std::fs::File;

pub type Buffer = RefCell<Option<BufWriter<File>>>;

thread_local! {
    pub static BUF: Buffer = RefCell::new(None);
}

#[macro_export] macro_rules!
logfile {
    ( $file:expr ) => {                 
        glob_replace!(
            $crate::tofile::BUF,
            Some(std::io::BufWriter::new($file))
        )
    };
    ( $capacity:expr, $file:expr ) => {       
        glob_replace!(
            $crate::tofile::BUF,
            Some(std::io::BufWriter::with_capacity($capacity, $file))
        )              
    };
}

#[macro_export]
#[cfg(feature = "no_auto_flush")] macro_rules!
auto_flush { ( $buf:expr ) => {} }

#[macro_export]
#[cfg(not(feature = "no_auto_flush"))] macro_rules!
auto_flush { ( $buf:expr ) => { $buf.flush().ok(); } }

#[macro_export] macro_rules!
tofile_writelog_inner_helper { ( $msg:expr ) => {
    glob_access!( 
            $crate::tofile::BUF,
            buf,
            { 
                match buf.as_mut() {
                    Some(buf) => {                        
                        use $crate::tofile::Write;
                        writeln!(buf, "{}", $msg).ok(); 
                        auto_flush!(buf);
                        buf 
                    },
                    None => {
                        panic!("Macro like `debug!()` should follow \
                                AFTER macro `logfile!(file)`. \
                                The log file has not been initialized.")
                    },
                }
            }
        );
    }
}

#[macro_export]
#[cfg(not(feature = "custom_writelog_inner"))] macro_rules!
writelog_inner { ( $msg:expr ) => { tofile_writelog_inner_helper!($msg); } }

#[macro_export] macro_rules!
logflush { () => { 
        glob_access!(
            $crate::tofile::BUF,
            buf,
            { 
                match buf.as_mut() {
                    Some(buf) => {
                        use $crate::tofile::Write;
                        buf.flush().ok();
                        buf
                    },
                    None => {
                        panic!("Macro `logflush!()` should follow \
                                AFTER macro `logfile!(file)`. \
                                The log file has not been initialized.")
                    },
                }
            }
        );
    }
}


