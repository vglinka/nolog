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
                        buffer.1.pop_back();
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
                buffer.1.iter()
                    .rev()
                    // Write all msg stored in buffer
                    .for_each(|x| writelog_inner!(x));
                buffer.1.clear();
                buffer
            }
        );
        // Write current msg
        writelog_inner!($msg);
    };
}

#[macro_export] macro_rules!
writelog {
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
                    buffer.1.push_front(format!("{}", $msg));
                } else {
                    buffer.1.pop_back();
                    buffer.1.push_front(format!("{}", $msg));
                }                
                buffer
            }
        );
        
    };
}

