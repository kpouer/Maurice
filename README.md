# Maurice

## Description

This is an emulator of Thomson MO5.
This repository is a fork of the MO5 java emulator repository https://github.com/kpouer/marcel which was intially a fork
of the original repository which is available at https://sourceforge.net/projects/marcel/

Of course the licence remains BSD Licence and all the credit goes to the original author.
My goal was to learn Rust and I thought that porting this emulator would be a good way to do so.

## Installation

# Via Cargo

1. Install rust https://www.rust-lang.org/
2. cargo install maurice

# Via releases

1. Some compiled binaries are available in Github releases

# Web version

This version runs using WASM32 and is available online at https://kpouer.github.io/Maurice/ and runs in any browsers (tested Chrome and derived browsers, Firefox and Safari).
However, while it is starting on a mobile browser you cannot really use it because of limitations of the touchscreen.

## Commands

### Keyboard

F7 : Soft Reset
F8 : Hard Reset

## Loading tapes

It is possible to load tapes by pressing F2 and selecting a .k7 file.
Then usually you have to type "load" and press enter. Then "run" and press enter.

## Showcase

### Boot
![Boot](media/boot.png)

### Arkanoid
![Boot](media/arkanoidanimated.png)
![Boot](media/arkanoid.png)

### Aigle d'or
![Boot](media/aigledor.png)
![Boot](media/aigledoranimated.png)

## Dependencies

This project depends on
- console_error_panic_hook (https://crates.io/crates/console_error_panic_hook)
- chrono (https://crates.io/crates/chrono)
- clap (https://crates.io/crates/clap)
- cpal (https://crates.io/crates/cpal)
- egui (https://crates.io/crates/egui)
- egui-file-dialog (https://crates.io/crates/egui-file-dialog)
- env_logger (https://crates.io/crates/env_logger)
- log (https://crates.io/crates/log)
- rust-embed-for-web (https://crates.io/crates/rust-embed-for-web)
- web-sys (https://crates.io/crates/web-sys)
- web-time (https://crates.io/crates/web-time)
