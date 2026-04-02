use crate::message::Message;

#[cfg(not(target_arch = "wasm32"))]
use crate::sender;

#[cfg(not(target_arch = "wasm32"))]
use tokio_schedule::{every, Job};

// Core gatling logic that can be shared - make it public
pub struct GatlingCore {
    pub messages_nb: i32,
    pub msg_sec: u32,
    pub server_url: String,
    pub kafka_topic: String,
    pub auth_token: String,
}

impl GatlingCore {
    pub fn new(messages_nb: i32, msg_sec: u32, server_url: String, kafka_topic: String, auth_token: String) -> Self {
        Self {
            messages_nb,
            msg_sec,
            server_url,
            kafka_topic,
            auth_token,
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
        log::info!("Gatling will repeat {} message(s) every {} seconds", self.messages_nb, self.msg_sec);
        let task = every(self.msg_sec).seconds().perform(|| { gatling_execute(self)} );
        task.await;
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
            if !self.auth_token.is_empty() {
                headers.set("Authorization", &format!("Bearer {}", self.auth_token))?;
            }

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

#[cfg(not(target_arch = "wasm32"))]
pub async fn gatling_execute(core: &GatlingCore) {
    log::info!("Executing Gatling task");

    let messages = core.generate_messages();

    if core.kafka_topic != "" {
        log::info!("Sending messages to Kafka topic {}", core.kafka_topic);
        sender::kafka::send_messages_to_kafka(messages, &core.kafka_topic).await.unwrap();
    } else {
        log::info!("Sending {} messages to {}", messages.len(), core.server_url);
        sender::wamserver::send_messages_to_wam_server(messages, &core.server_url).await.unwrap();
    }
}