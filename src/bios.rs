use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "bios"]
pub(crate) struct Bios;