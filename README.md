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
- chrono (https://crates.io/crates/chrono)
- cpal (https://crates.io/crates/cpal)
- env_logger (https://crates.io/crates/env_logger)
- log (https://crates.io/crates/log)
- rust-embed (https://crates.io/crates/rust-embed)
- egui (https://crates.io/crates/egui)
- egui-file-dialog (https://crates.io/crates/egui-file-dialog)
