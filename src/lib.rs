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

#![deny(unsafe_code)]

pub mod disabled;

#[cfg(any(debug_assertions, feature = "release"))]
pub mod core;

#[cfg(
    any(
        all(debug_assertions, feature = "tofile"),
        all(feature = "release", feature = "tofile")
))]
pub mod tofile;

#[cfg(
    any(
        all(debug_assertions, feature = "logonly"),
        all(feature = "release", feature = "logonly")
))]
pub mod logonly;

#[cfg(
    any(
        all(debug_assertions, feature = "logonly", not(feature = "logmod")),
        all(feature = "release", feature = "logonly", not(feature = "logmod"))
))]
pub mod level_logonly;

#[cfg(
    any(
        all(debug_assertions, feature = "logcatch"),
        all(feature = "release", feature = "logcatch")
))]
pub mod logcatch;

#[cfg(
    any(
        all(debug_assertions, feature = "logmod"),
        all(feature = "release", feature = "logmod")
))]
pub mod logmod;

#[cfg(
    any(
        all(debug_assertions, feature = "logmod", not(feature = "logonly")),
        all(feature = "release", feature = "logmod", not(feature = "logonly"))
))]
pub mod level_logmod;

#[cfg(
    any(
        all(debug_assertions, feature = "logonly", feature = "logmod"),
        all(feature = "release", feature = "logonly", feature = "logmod")
))]
pub mod level_logonly_and_logmod;

#[cfg(
    any(
        all(debug_assertions, feature = "glob"),
        all(feature = "release", feature = "glob")
))]
pub mod glob;

#[cfg(
    any(
        all(debug_assertions, feature = "log_enabled"),
        all(feature = "release", feature = "log_enabled")
))]
pub mod log_enabled;

