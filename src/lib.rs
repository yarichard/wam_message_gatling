pub mod sender;
pub mod message;
pub mod gatling_core;

#[cfg(target_arch = "wasm32")]
mod wasm_binding;

#[cfg(target_arch = "wasm32")]
pub use wasm_binding::*;