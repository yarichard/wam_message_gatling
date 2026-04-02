use crate::gatling_core::GatlingCore;

// use super::*; // Is it normal to removing this ?????
use wasm_bindgen::prelude::*;

// Set up panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Configuration struct for WASM
#[wasm_bindgen]
pub struct GatlingConfig {
    core: GatlingCore,
}

#[wasm_bindgen]
impl GatlingConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(messages_nb: i32, msg_sec: u32, server_url: String, auth_token: String) -> Self {
        Self {
            core: GatlingCore::new(messages_nb, msg_sec, server_url, "".to_string(), auth_token),
        }
    }

    #[wasm_bindgen]
    pub async fn gatling_execute(&self) -> Result<String, wasm_bindgen::JsValue> {
        self.core.execute_wasm().await
    }

    #[wasm_bindgen(getter)]
    pub fn messages_nb(&self) -> i32 {
        self.core.messages_nb
    }

    #[wasm_bindgen(getter)]
    pub fn msg_sec(&self) -> u32 {
        self.core.msg_sec
    }

    #[wasm_bindgen(getter)]
    pub fn server_url(&self) -> String {
        self.core.server_url.clone()
    }
}

// Standalone function that can be called from JavaScript
#[wasm_bindgen]
pub async fn gatling_execute_standalone(messages_nb: i32, msg_sec: u32, server_url: String, auth_token: String) -> Result<String, wasm_bindgen::JsValue> {
    let core = GatlingCore::new(messages_nb, msg_sec, server_url, "".to_string(), auth_token);
    core.execute_wasm().await
}