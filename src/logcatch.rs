// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

pub use std::collections::VecDeque;
pub use std::cell::RefCell;

pub type BufferCapacity = usize;
pub type Buffer = RefCell<(BufferCapacity, VecDeque<String>)>;

pub const DEFAULT_CAPACITY: BufferCapacity = 10;


thread_local! {
    // Store 10 msg by default
    pub static LOG_CATCH: Buffer = RefCell::new((DEFAULT_CAPACITY, Default::default()));
}

#[macro_export] macro_rules!
logcatch {
    ( $new_capacity:expr ) => {
        glob_access!(
            $crate::logcatch::LOG_CATCH,
            buffer,
            {
                if buffer.0 > $new_capacity {
                    for _ in 0..(buffer.0 - $new_capacity) {
                        buffer.1.pop_front();
                    }
                }
                buffer.0 = $new_capacity;      
                buffer
            }
        );
    };
}

#[macro_export] macro_rules!
writelog_flush_helper {
    ( $msg:expr ) => {
        // Flush buffer
        glob_access!(
            $crate::logcatch::LOG_CATCH,
            buffer,
            {
                // Write all msg stored in buffer
                for msg in &buffer.1 {
                    writelog_inner!(msg);
                }
                buffer.1.clear();
                buffer
            }
        );
        // Write current msg
        writelog_inner!($msg);
    };
}

#[macro_export] macro_rules!
logcatch_writelog_helper {
    // crit msg
    ( [[crit]], [$msg_type:tt], $msg:expr ) => {
        writelog_flush_helper!($msg);
    };
    // error msg
    ( [[error]], [$msg_type:tt], $msg:expr ) => {
        writelog_flush_helper!($msg);
    };
    // others msg
    ( [$level:tt], [$msg_type:tt], $msg:expr ) => {       
        glob_access!(
            $crate::logcatch::LOG_CATCH,
            buffer,
            { 
                if  buffer.1.len() < buffer.0 {
                    buffer.1.push_back(format!("{}", $msg));
                } else {
                    buffer.1.pop_front();
                    buffer.1.push_back(format!("{}", $msg));
                }                
                buffer
            }
        );
        
    };
}

#[macro_export]
#[cfg(not(feature = "custom_writelog"))] macro_rules!
writelog { ( $($a:tt)* ) => { logcatch_writelog_helper!($($a)*); }; }

