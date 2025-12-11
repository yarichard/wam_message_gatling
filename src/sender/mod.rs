#[cfg(not(target_arch = "wasm32"))]
pub mod kafka;

pub mod wamserver;