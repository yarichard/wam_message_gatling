use wam_message_gatling::message::Message;
use wam_message_gatling::sender::wamserver::{send_to_wam_server, send_messages_to_wam_server};

#[tokio::test]
async fn test_send_to_wam_server_invalid_url() {
    // Set an invalid URL to test error handling
    let msg = Message {
        id: 1,
        text: "Test".to_string(),
        user_id: 42,
    };
    let result = send_to_wam_server(&msg, &"http://localhost:9999".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_messages_to_wam_server_empty() {
    // Set a valid URL (but no server running, just test error handling)
    let messages = Vec::new();
    let result = send_messages_to_wam_server(messages, &"http://localhost:9999".to_string()).await;
    assert!(result.is_ok());
}