// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

pub use std::cell::RefCell;

thread_local! {
    pub static LOG_ENABLED: RefCell<bool> = RefCell::new(false);
}
