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

#[cfg(
    all(
        not(feature = "logonly"),
        not(feature = "logmod"),
))]
pub mod level;

pub mod level_inner;
pub mod format;
pub mod msg_render;
pub mod writelog;
pub mod customization;
pub mod lvl_header;
pub mod color;
pub mod separator;
pub mod indent;
pub mod indent_base;
pub mod newline;

