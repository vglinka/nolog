## nolog logger

Convenient and 'beautiful by default' logger for debugging your programs.
Easy to use, you don't need to learn anything to start using it.
Zero deps. No unsafe (by `#![deny(unsafe_code)]`).

![nolog](https://raw.githubusercontent.com/vglinka/nolog/main/assets/term.gif)

<p align="center">
<a href="https://github.com/vglinka/nolog/blob/main/examples/novus/main.rs" target="_blank">See this example code</a>
</p>

## nolog features
- In most cases `nolog` uses a `format_args!()` (that avoids heap
  allocations) and compile-time level filtering by Cargo features.
- Filtering by module path (`logmod` feature).
- Display messages only from a selected section of code (`logonly` feature).
- Smart logging: hide all messages, show the previous `Х` messages
  if an `error` or `crit` level message was triggered (`logcatch` feature).
- A custom indent can be added to the message, as well as a number
  of blank lines before and after it. You can use variables (eg loop
  counter) to set the indent for the selected message.
- Easily adjust and disable all indents and newlines with features.
- Сustomization: you can create your own color scheme for the logger.
- Support for named format arguments: `info!("{line_count} lines.");`.
- Support for `key => value` syntax: `info!("{server}" => "{ip}");`
- Automatically disabled in the release build: `cargo run --release`.
  If you want the log to be enabled in the release build, then use
  `release` feature: `nolog = {version = "*", features = ["release"]}`.
- All levels are disabled by default. Turning on the `debug` level also
  turns on the levels above it: `info`, `warn`, `error`, `crit`.
  Level can be enabled using the console:
  `cargo run --features debug` or in `Cargo.toml`:
  `nolog = {version = "*", features = ["debug"]}`. 
  To enable all levels: `cargo run --features trace`.
- By default, the log is writting to `stderr`. You can log to a file with
  `tofile` feature. You may set the buffer size. Automatic flush after
  each message will be used. If you want wait for the buffer to fill
  or to do it manually with `logflush!()` then use `no_auto_flush` feature.
- Custom output redirection. For example, to a `file` and to `stderr`
  at the same time. An example is at the very bottom of the page.
- You can add a timestamp like `[2022-07-10 06:49:33.646361181 UTC]`
  using a third party library you like. An example is below.
- Support for chaining multiple messages into one (they must all be
  of the same type: `usual` or `key-value`): 
```rust
info!(
        "{server}" => "{ip}";
        "Status"   => "{server_check_result}";
);
```

## Using nolog

**Cargo.toml**

```toml
[dependencies]
nolog = { version = "1", features = [] }

[features]
nolog_setup = []

# example `classic`
#nolog_setup = ["nolog/show_lvl_header_kv", "nolog/indent_ignore_all", "nolog/newline_ignore", "nolog/location_style_classic", "nolog/sep_colon"]

# example `classic_plain`
#nolog_setup = ["nolog/plain", "nolog/show_lvl_header_kv", "nolog/indent_ignore_all", "nolog/newline_ignore", "nolog/location_style_classic", "nolog/sep_colon"]

trace = ["nolog/trace", "nolog_setup"]
debug = ["nolog/debug", "nolog_setup"]
info  = ["nolog/info",  "nolog_setup"]
warn  = ["nolog/warn",  "nolog_setup"]
error = ["nolog/error", "nolog_setup"]
crit  = ["nolog/crit",  "nolog_setup"]

logonly  = ["nolog/logonly"]
logcatch = ["nolog/logcatch"]
logmod   = ["nolog/logmod"]
```

**main.rs**

```rust
#[macro_use]
extern crate nolog;

fn main() {
    trace!("line_count: {}", 42);
    debug!("line_count: {}", 42);
    info!("line_count: {}", 42);
    warn!("line_count: {}", 42);
    error!("line_count: {}", 42);
    crit!("line_count: {}", 42);
}
```

[This example on GitHub](https://github.com/vglinka/nolog/tree/main/examples/min_example).

`nolog` has the same syntax as most loggers based on the `log` crate.
`nolog` extends the `log` crate syntax by adding new features.
However, `nolog` is not based on `log` crate, it just has the same
macro names.

Therefore, switching to `nolog` will require minimal changes in the code.
In fact, this boils down to a change in `Cargo.toml`
and line `extern crate nolog;`.

Then:

```sh
cargo run --features trace
```

Or, for example

```sh
# The output will be empty because there are no logonly
# blocks, etc. in the code.
# This is just to demonstrate the use of several features.
cargo run --features trace,logonly,logcatch,logmod
```

It's the same but noisier

```sh
# The output will be empty because there are no logonly
# blocks, etc. in the code.
# This is just to demonstrate the use of several features.
cargo run --features nolog/trace,nolog/logonly,nolog/logcatch,nolog/logmod
```

**Result:**

![example](https://raw.githubusercontent.com/vglinka/nolog/main/assets/min.png)

## Tofile. Writing log entries to a file

**Cargo.toml**

```toml
#...
[dependencies]
nolog = { version = "1", features = ["tofile"] }
#...
```

**main.rs**

```rust
use std::fs::OpenOptions;
use std::io::{self, Read};
use std::path::PathBuf;

#[macro_use]
extern crate nolog;

fn main() -> io::Result<()> {
    let path = PathBuf::from("log.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        //^^^^^^^ truncate the file to 0 length if it already exists.
        //.append(true)
        .open(&path)?;
        
    // Initialization
    // Don't use macros like `debug!("msg");` before initialization.
    logfile!(file);
    
    trace!("Hello from file!");
        
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("\n      -- In {path:?} --");
    println!("{contents}");
    
    Ok(())
}
```

[This example on GitHub](https://github.com/vglinka/nolog/tree/main/examples/tofile).

Optionally, you can set the buffer size.

```rust
    // Buffer `std::io::BufWriter` with capacity: 8000 bytes.
    logfile!(8000, file);
```

The default is to automatically flush after each message.
If you want wait for the buffer to fill or to do it manually
with `logflush!()` then use `no_auto_flush` feature.

**Cargo.toml**

```toml
#...
[dependencies]
nolog = { version = "1", features = ["tofile", "no_auto_flush"] }
#...
```

Then use `logflush!()` to flush the log manually.

**main.rs**

```rust
...
    // Initialization
    // Don't use macros like `debug!("msg");` before initialization.
    logfile!(8000, file);
    
    trace!("Hello from file!");
    logflush!();     
...
```

## How to add a timestamp

You can add a timestamp like `[2022-07-10 06:49:33.646361181 UTC]`
using a third party library you like.

For this example, we will use `chrono` crate.

**Cargo.toml**

```toml
#...
[dependencies]
nolog = { version = "1", features = [] }
chrono = "0.4"

[features]
custom_leading    = ["nolog/custom_leading"]
custom_trailing   = ["nolog/custom_trailing"]
custom_before_msg = ["nolog/custom_before_msg"]
custom_after_msg  = ["nolog/custom_after_msg"]

nolog_setup = ["custom_leading"]
#...
```

We have 4 options here:
- "custom_leading" - `<TIMESTAMP>CRIT⧽ msg [5] src/main.rs`
- "custom_trailing" - `CRIT⧽ msg [5] src/main.rs<TIMESTAMP>`
- "custom_before_msg" - `CRIT⧽ <TIMESTAMP>msg [5] src/main.rs`
- "custom_after_msg" - `CRIT⧽ msg<TIMESTAMP> [5] src/main.rs`

Log entry structure:

usual:
`<indents><custom_leading><lvlheader><sep><custom_before_msg><msg><custom_after_msg><location><custom_trailing>`

key-value:
`<indents><custom_leading><lvlheader><sep_kv><custom_before_msg><key><sep_key><value_indent><value><custom_after_msg><location><custom_trailing>`

Here is an example:

**main.rs**

```rust
#[macro_use]
extern crate nolog;

#[macro_use] 
pub mod logger_setup {
    #[macro_export]
    #[cfg(feature = "custom_leading")] macro_rules!
    //               ^^^^^^^^^^^^^^
    custom_leading { 
        // usual
        ( $level:tt, $indent:expr, $($msg:expr),* ) => {
            format_args!("[{}] ", chrono::Utc::now())
            
        };
        // key-value
        ( $level:tt, $indent:expr, $($key:expr),* => $($value:expr),* ) => {
            format_args!("[{}] ", chrono::Local::now())
        };
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

fn main() {
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();   
}
```

[This example on GitHub](https://github.com/vglinka/nolog/tree/main/examples/timestamp).

Output:

```sh
[2022-09-07 09:22:09.150921578 UTC] CRIT⧽ Hello from main! This is usual msg. [34] src/main.rs
[2022-09-07 12:22:09.150973037 +03:00] Other⧽ Hello from other mod! This is key-value msg. [29] src/main.rs

```

With `classic` style:

```sh
[2022-09-07 09:29:45.859185734 UTC] CRIT: Hello from main! This is usual msg. [src/main.rs 34:5]
[2022-09-07 12:29:45.859225186 +03:00] CRIT: Other: Hello from other mod! This is key-value msg. [src/main.rs 29:9]
```

## Styles

#### Default
```toml
nolog_setup = []
```

#### `classic`

```toml
nolog_setup = ["nolog/show_lvl_header_kv", "nolog/indent_ignore_all", "nolog/newline_ignore", "nolog/location_style_classic", "nolog/sep_colon"]
```

#### `classic_plain`

```toml
nolog_setup = ["nolog/plain", "nolog/show_lvl_header_kv", "nolog/indent_ignore_all", "nolog/newline_ignore", "nolog/location_style_classic", "nolog/sep_colon"]
```

![styles](https://raw.githubusercontent.com/vglinka/nolog/main/assets/style.gif)

## Chaining

Messages in a chain should all be of the same type: `usual` or `key-value`

**ususal**

```rust
debug!(
    "Planet {name} thinks...";
    "Planet {name} thinks...";
);
```

**key-value**

```rust
debug!(
    "{server}" => "{ip}";
    "Status"   => "{server_check_result}";
);
```

## Logmod. Filtering by module path

Add it as early as possible in the code:

```rust
logmod!(
    [  ] main,
    [!=] crate::other2,
);
```

- `[]`   - Include a module and all its submodules.
- `[=]`  - the same (Include a module and all its submodules).
- `[!]`  - Exclude a module and all its submodules.
- `[==]` - Include only this module without submodules.
- `[!=]` - Exclude only this module without submodules.

Then

`cargo run --features trace,logmod`

## Logonly. Display messages only from a selected section of code

This is useful for debugging to get messages from just a small
piece of code. 

```rust
logonly!(
    let universe = [0;3];
    crit!("The Universe was created with a lifetime of {} days.", universe.len());
    error!("Uncontrolled evolutionary processes have begun on the planet {planet_name}.");
);
```

Then

`cargo run --features trace,logonly`

You can use any brackets

`logonly!()`, `logonly!{}`, `logonly![]`.

You can use multiple `logonly!()` blocks. Messages will be displayed from all.

It won't break your code when the logger turns off in release build.
So you can leave these blocks in the code.

When disabled, the definition of this macro will be replaced with
the following:

```rust
logonly { ( $($a:tt)* ) => { $($a)* }; }
```

It simply writes down the code it received.

## Logcatch. Smart logging

Hide all messages, show the previous `Х` messages if an `error` or `crit`
level message was triggered.

By default `X=10`. You can change this anywhere in the code.

```rust
// This will take effect for the code below.
logcatch!(2); // now X=2
```

To enable this feature, use:

`cargo run --features trace,logcatch`

Each new line created with `newline!()` or `->[_,1,1]`(about
what it will be below) counts as a separate message.

## Quick disable and enable messages

You can disable individual messages without removing them from the code.
A macro like `debug!([_]; "msg")` will expand into an empty tuple `()`.

```rust
// on
info!([#]; "New {name} on planet {planet_name}.");

// off
info!([_]; "{repr}" => "{name} says: {speech}");
```

You can use any options you like:

**On:**
`[#]`, `[x]`, `[v]`, `[+]`, `[on]`, `[true]`, `[your_var]`

**Off:**
`[ ]`, `[_]`, `[-]`, `[off]`, `[false]`, `[your_var]`

`your_var` should be `bool`.

To change states, you need to change only one character:

`[_]` --> `[#]`.

This also works with chained messages, but disables the entire chain.
You can't turn off a single message in a chain.

```rust
crit!([_];
    "The answer is {answer}."; 
    "Planet {planet_name} started watching TV.";
);
```

You can turn off the action of block `logonly`. This will not affect
the code, the effect is as if macro `logonly` was not in this place.

```rust
logonly!{[_];
    crit!("The answer is {answer}.");
    let x = 42;
}
```

This way you can leave `logonly!()` in the code and if it is required
in the future just enable it.

#### Variables and expressions

If necessary, you can control messages using variables and expressions.

```rust
let my_log_enabled = true;
crit!([my_log_enabled]; "The planet {} has been destroyed.", self.name);
```

```rust
let status = "ok";
crit!([(status == "ok")]; "The planet {} has been destroyed.", self.name);
//     ^              ^
//     Add parentheses
```

```rust
fn is_message_show_fn () -> bool { false }
...
crit!([(is_message_show_fn())]; "The planet {} has been destroyed.", self.name);
//     ^                    ^
//     Add parentheses
```

## Indentation and new lines

### new lines

`newline!(2);` - It will simply write the passed number of new lines to the log.

### Indentation

Indents are of several types:

#### Base indent

Base indent will be added to every line.
- Default for all: `6` indents. One indent equals one space.

You can change `base indent` with cargo features:

- `indent_base_zero`
- `indent_base_one`
- `indent_base_two`
- `indent_base_three`
- `indent_base_four`
- `indent_base_five`
- `indent_base_seven`
- `indent_base_eight`
- `indent_base_nine`
- `indent_base_ten`

For example in `Cargo.toml`: 

```toml
nolog_setup = ["nolog/indent_base_zero"]

trace = ["nolog/trace", "nolog_setup"]
```

#### The indent of the selected message

- Default for `usual`: 0
- Default for `key-value`: 6

The default indentation is used if no value has been provided by the user.

You can specify indentation in the following way:

`crit!(->[X,Y,Z] "msg");`

- `X` - Indents.
- `Y` - Add `Y` blank lines before message (same effect as `newline!(Y)`).
- `Z` - Add `Z` blank lines after message.

All of these arguments are optional:

```rust
crit!("msg");
crit!(->[1] "msg");
crit!(->[6,1] "msg");
crit!(->[1,2,3] "msg");
crit!([#]; ->[1,2,3] "msg");
```

If you want to add blank lines and leave the default indentation:

```rust
crit!(->[_,1] "msg");
crit!(->[_,_,2] "msg");
```

The same works for each message in the chain.

```rust
debug!(
    ->[2]   "Planet {name} thinks...";
    ->[_,1] "Planet {name} thinks...";
            "Planet {name} thinks...";
);
```

`key => value` ​​have an indentation of `6` by default, but you can reset
it by setting it to zero.

```rust
error!(->[0] "{name}" => "{}!! Oh, yeaaaah!", 2*3*7);
```

Or you can do it via `Cargo.toml` for all messages.

- `indent_kv_default_zero`
- `indent_kv_default_one`
- `indent_kv_default_two`
- `indent_kv_default_three`
- `indent_kv_default_four`
- `indent_kv_default_five`
- `indent_kv_default_seven`
- `indent_kv_default_eight`
- `indent_kv_default_nine`
- `indent_kv_default_ten`

For example in `Cargo.toml`:
```toml
nolog_setup = ["nolog/indent_kv_default_zero"]

trace = ["nolog/trace", "nolog_setup"]
```

Key-values ​​have the additional ability to set indentation not only
for the key, but also for the value.

```rust
debug!(
    "{server}" =>       "{ip}";
    "Status"   => ->[3] "{server_check_result}";
);
```

This allows you to get nice aligned output if you want.

![indent](https://raw.githubusercontent.com/vglinka/nolog/main/assets/indent_1.png)

#### Indent variables

You can use variables to set the indentation and add blank lines.

```rust
for i in 0..2 {
    warn!(->[i,i,i] "msg");
}
```

#### Ignore all indents

Ignore all types of indentation.

```toml
nolog_setup = ["nolog/indent_ignore_all"]
```

![indent](https://raw.githubusercontent.com/vglinka/nolog/main/assets/indent_2.png)

#### Ignore all newlines

```toml
nolog_setup = ["nolog/newline_ignore"]
```

## Colors

`nolog` colored by default, use this feature for plain output:

```toml
nolog_setup = ["nolog/plain"]
```

## Level headers

```toml
nolog_setup = ["nolog/show_lvl_header_kv"]
```

Show level name for key-value:
```sh
CRIT: Key: value [src/main.rs 90:5]`
^^^^
```

It's disabled by default: 
```sh
Key: value [src/main.rs 90:5]
```

## Don't disable logger in release build

```toml
nolog = { version = "1", features = ["release"] }
```

## Location

Don't show location (like `[src/main.rs 155:9]`)

```toml
nolog_setup = ["nolog/location_hide"]
```

Style like this: `[src/main.rs 155:9]`

```toml
nolog_setup = ["nolog/location_style_classic"]
```

![location](https://raw.githubusercontent.com/vglinka/nolog/main/assets/location.gif)

## Separator

Default = "⧽ "

- ": "

```toml
nolog_setup = ["nolog/sep_colon"]
```

- " "

```toml
nolog_setup = ["nolog/sep_space"]
```


- ""

```toml
nolog_setup = ["nolog/sep_hide"]
```

## Custom color scheme

You can create your own color scheme for the logger.

**Cargo.toml**

```toml
#...
[dependencies]
nolog = { version = "1", features = [] }

[features]
custom_colors = ["nolog/custom_colors"]
nolog_setup = ["custom_colors"]
#...
```

Here is an example:

**main.rs**

```rust
#[macro_use]
extern crate nolog;

#[macro_use] 
pub mod logger_setup {
    #[macro_export]
    #[cfg(feature = "custom_colors")] macro_rules!
    //               ^^^^^^^^^^^^^
    color {
        ( [trace] ) => { "\x1B[34m"                    };
        ( [debug] ) => { "\x1B[36m"                    };
        ( [info]  ) => { "\x1B[32m"                    };
        ( [warn]  ) => { "\x1B[33m"                    };
        ( [error] ) => { "\x1B[31m"                    };
        ( [crit]  ) => { "\x1B[35m"                    };
        ( [sep]   ) => { "\x1B[1m\x1B[2m"              }; // +bold +dim
        ( [msg]   ) => { ""                            }; // default term font color
        ( [from]  ) => { "\x1B[90m\x1B[3m"             }; // `[src/main.rs 101:5]` in `location_style_classic`
        ( [sep2]  ) => { "\x1B[90m\x1B[2m"             }; // sep2 in default style
        ( [sep3]  ) => { "\x1B[90m\x1B[2m"             }; // sep3 in default style
        ( [line]  ) => { "\x1B[38;5;67m\x1B[1m\x1B[2m" }; // line number in default style
        ( [key]   ) => { "\x1B[3m\x1B[1m"              }; // +italic +bold 
        ( [value] ) => { "\x1B[3m"                     }; // +italic
        ( [rm]    ) => { "\x1B[0m"                     }; // remove previous colors
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

fn main(){
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();   
}
```

[This example on GitHub](https://github.com/vglinka/nolog/tree/main/examples/custom_color_scheme).

## Custom output redirection

It is possible to redirect output. For example, we will redirect
output to stderr and to a file at the same time. The limitation is that
output to stderr will be the same as to a file, it will not be colorized.

**Cargo.toml**

```toml
#...
[dependencies]
nolog = { version = "1", features = [] }

[features]
nolog_setup = [
    "custom_writelog_inner",
    "nolog/tofile"
]
custom_writelog_inner = ["nolog/custom_writelog_inner"]
#...
```

Here is an example:

**main.rs**

```rust
use std::fs::OpenOptions;
use std::io::{self, Read};
use std::path::PathBuf;

#[macro_use]
extern crate nolog;

// use `cargo run --features trace`

#[macro_use] 
pub mod logger_setup {
    #[macro_export]
    #[cfg(feature = "custom_writelog_inner")] macro_rules!
    writelog_inner { ( $msg:expr ) => {
            eprintln!("{}", $msg); // write to stderr
            tofile_writelog_inner_helper!($msg); // write to file
        }
    }
}

mod other {
    pub fn from_other_mod() -> () {
        crit!(->[0] "Other" => "Hello from other mod! This is key-value msg.");
    }
}

fn main() -> io::Result<()> {
    let path = PathBuf::from("log.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        //^^^^^^^ truncate the file to 0 length if it already exists.
        //.append(true)
        .open(&path)?;
        
    // Initialization
    // Don't use macros like `debug!("msg");` before initialization.
    logfile!(file);
    
    eprintln!("\n-- From eprintln: --");
    crit!("Hello from main! This is usual msg.");
    other::from_other_mod();
    
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("\n-- In {path:?} --");
    println!("{contents}");
    
    Ok(())
}
```

[This example on GitHub](https://github.com/vglinka/nolog/tree/main/examples/custom_output_redirection).

Output:

```sh
-- From eprintln: --
CRIT: Hello from main! This is usual msg. [main.rs 54:5]
CRIT: Other: Hello from other mod! This is key-value msg. [main.rs 34:9]

-- In "log.txt" --
CRIT: Hello from main! This is usual msg. [main.rs 54:5]
CRIT: Other: Hello from other mod! This is key-value msg. [main.rs 34:9]

```

## Other customization options

`nolog` has other customization options not described here, since
it is unlikely that they will be in demand by a wide range of users.
Their use is similar to that described above.
You can see the full up-to-date list in
[Cargo.toml](https://github.com/vglinka/nolog/tree/main/Cargo.toml).

## Logging in tests

Logging in tests works exactly the same, except that Rust test programs
hide standard output of successful tests.

Use the following code to see the output of successful tests.

```sh
cargo test --features trace -- --nocapture
```

The output of failed tests will be displayed anyway.

```sh
cargo test --features trace
```

## Changelog

- **1.0.10 - 1.0.11** – Minor changes, an example with output redirection has been added.
- **1.0.1 - 1.0.9** – Small changes in Readme etc.
- **1.0.0** – Release. Completely rewritten.
- **0.1.1-0.2.3** – Early versions.
