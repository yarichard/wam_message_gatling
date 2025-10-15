pub mod sender;
pub mod message;

use message::Message;

// Core gatling logic that can be shared - make it public
pub struct GatlingCore {
    pub messages_nb: i32,
    pub msg_sec: u32,
    pub server_url: String,
}

impl GatlingCore {
    pub fn new(messages_nb: i32, msg_sec: u32, server_url: String) -> Self {
        Self {
            messages_nb,
            msg_sec,
            server_url,
        }
    }

    pub fn generate_messages(&self) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        for n in 0..self.messages_nb {
            let msg = Message {
                id: n,
                text: format!("Message from Gatling number {}", n),
                user_id: 1,
            };
            messages.push(msg);
        }
        messages
    }

    pub fn get_delay_ms(&self) -> u64 {
        if self.msg_sec > 0 {
            (1000.0 / self.msg_sec as f64) as u64
        } else {
            0
        }
    }

    // Standard execution using reqwest (for main.rs)
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn execute_standard(&self) -> Result<String, Box<dyn std::error::Error>> {
        log::info!("Executing Gatling task");
        
        let messages = self.generate_messages();
        log::info!("Sending {} messages to {}", messages.len(), self.server_url);

        sender::wamserver::send_messages_to_wam_server(messages, &self.server_url).await?;
        Ok("Messages sent successfully".to_string())
    }

    // WASM execution using web APIs
    #[cfg(target_arch = "wasm32")]
    pub async fn execute_wasm(&self) -> Result<String, wasm_bindgen::JsValue> {
        web_sys::console::log_1(&"Executing Gatling task".into());
        
        let messages = self.generate_messages();
        web_sys::console::log_1(&format!("Sending {} messages to {}", messages.len(), self.server_url).into());

        self.send_messages_wasm(messages).await?;
        Ok("Messages sent successfully".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    async fn send_messages_wasm(&self, messages: Vec<Message>) -> Result<(), wasm_bindgen::JsValue> {
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
        use wasm_bindgen::{JsValue, JsCast};
        use serde_json;
        
        let window = web_sys::window().unwrap();
        let delay_ms = self.get_delay_ms() as i32;
        
        for message in messages {
            // Create request options
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);

            // Set headers
            let headers = Headers::new()?;
            headers.set("Content-Type", "application/json")?;

            // Serialize message
            let body = serde_json::to_string(&message)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;
            
            opts.set_body(&JsValue::from_str(&body));
            opts.set_headers(&headers);

            // Create and send request
            let url = format!("{}/api/message", self.server_url);
            let request = Request::new_with_str_and_init(&url, &opts)?;
            
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
            let resp: Response = resp_value.dyn_into().unwrap();

            if !resp.ok() {
                return Err(JsValue::from_str(&format!("HTTP Error: {}", resp.status())));
            }

            // Add delay between messages
            if delay_ms > 0 {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay_ms)
                        .unwrap();
                });
                JsFuture::from(promise).await?;
            }
        }

        Ok(())
    }
}

// WASM-specific code below
#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use super::*;
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
        pub fn new(messages_nb: i32, msg_sec: u32, server_url: String) -> Self {
            Self {
                core: GatlingCore::new(messages_nb, msg_sec, server_url),
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
    pub async fn gatling_execute_standalone(messages_nb: i32, msg_sec: u32, server_url: String) -> Result<String, wasm_bindgen::JsValue> {
        let core = GatlingCore::new(messages_nb, msg_sec, server_url);
        core.execute_wasm().await
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::*;