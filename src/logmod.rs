// Copyright (c) 2022-present Vadim Glinka
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.

pub use std::cell::RefCell;
pub use std::collections::HashSet;

pub type Enabled = HashSet<String>;
pub type Disabled = HashSet<String>;
pub type Included = HashSet<String>;
pub type Excluded = HashSet<String>;

// (enabled, disabled, included, excluded)
//                     |_________________|
//                            cache
pub type ModList = RefCell<(Enabled, Disabled, Included, Excluded)>;

thread_local! {
    pub static MOD_LIST: ModList = RefCell::new(Default::default());
}

#[macro_export] macro_rules!
logmod {
    ( $( $([$($n:tt)?])? $p:path, )* ) => {
        let mut enabled  = std::collections::HashSet::<String>::new();
        let mut disabled = std::collections::HashSet::<String>::new();
        let mut included = std::collections::HashSet::<String>::new();
        let mut excluded = std::collections::HashSet::<String>::new();
        $(
            mod_path_parse!([$($($n)?)?], $p, enabled, disabled, included, excluded);
        )*;
        
        glob_replace!(
            $crate::logmod::MOD_LIST,
            (enabled, disabled, included, excluded)
        );
    };
    
    // add the trailing colon
    ( $( $([$($n:tt)?])? $p:path ),* ) => { logmod!($( $([$($n)?])? $p ),*,) };
}

#[macro_export] macro_rules!
mod_path_parse {
    ( [  ], $p:path, $enabled:expr, $disabled:expr, $included:expr, $excluded:expr ) => {
        $enabled.insert(mod_path_normalyze!($p));
        $included.insert(mod_path_normalyze!($p));
    };

    ( [= ], $p:path, $enabled:expr, $disabled:expr, $included:expr, $excluded:expr ) => {
        mod_path_parse!([  ], $p, $enabled, $disabled, $included, $excluded)
    };

    ( [! ], $p:path, $enabled:expr, $disabled:expr, $included:expr, $excluded:expr ) => {
        $disabled.insert(mod_path_normalyze!($p));
        $excluded.insert(mod_path_normalyze!($p));
    };

    ( [==], $p:path, $enabled:expr, $disabled:expr, $included:expr, $excluded:expr ) => {
        $included.insert(mod_path_normalyze!($p));
    };

    ( [!=], $p:path, $enabled:expr, $disabled:expr, $included:expr, $excluded:expr ) => {
        $excluded.insert(mod_path_normalyze!($p));
    };
}

#[macro_export] macro_rules!
mod_path_normalyze {
    ( $p:path ) => {
        {
            let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("").trim().to_lowercase().replace("-", "_");
            let mut path = stringify!($p).trim().to_lowercase().replace("crate", &app_name).replace("main", &app_name);
            if path.starts_with("::") {
                path = path.replacen("::", &format!("{app_name}::"), 1);
            }
            path
        }
    };
}

#[macro_export] macro_rules!
logmod_helper {
    ( $($msg:tt)* ) => {
        glob_access!(
            $crate::logmod::MOD_LIST,
            mod_list,
            { 
                let path = module_path!().to_lowercase();

                //   tuple     HashSet   HashSet   HashSet   HashSet
                // mod_list = (enabled, disabled, included, excluded)
                //                0         1         2         3
                //                                |_________________|
                //                                       cache

                // in excluded
                if mod_list.3.contains(&path) {
                    return mod_list
                }
                // in included
                if mod_list.2.contains(&path) {
                    $($msg)*; // print msg
                    return mod_list;
                }

                // "appname::mod_name::other_mod" --> path_len = 2
                let path_len = path.trim_matches(':').matches("::").count();
                let mut path_tmp = path.clone();

                for _ in 0..path_len {               
                    // "appname::mod_name::other_mod" --> "appname::mod_name"
                    path_tmp = match path_tmp.rsplit_once("::") {
                        Some(v) => { v.0.into() },
                        None => unreachable!(),
                    };
                    // path_tmp in `disabled`
                    if mod_list.1.contains(&path_tmp) {
                        // add `path` and `path_tmp` to excluded
                        mod_list.3.insert(path);
                        mod_list.3.insert(path_tmp);
                        return mod_list;
                    // path_tmp in `enabled`
                    } else if mod_list.0.contains(&path_tmp) {
                        $($msg)*; // print msg
                        // add `path` and `path_tmp` to included
                        mod_list.2.insert(path);
                        mod_list.2.insert(path_tmp);
                        return mod_list;
                    // path_tmp is not in `included`
                    } else if !mod_list.2.contains(&path_tmp) {
                        // add `path_tmp` to excluded
                        mod_list.3.insert(path_tmp.clone());
                    }
                }
                
                // add `path` to excluded
                mod_list.3.insert(path);
                
                mod_list
            }
        );
    };
}



