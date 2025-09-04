#[cfg(not(target_family = "wasm"))]
pub mod args;
pub(crate) mod bios;
pub mod gui;
pub mod hardware;
pub mod raw_image;

pub(crate) type int = i32;
