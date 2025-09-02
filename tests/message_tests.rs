use wam_message_gatling::message::Message;

#[test]
fn test_message_display() {
    let msg = Message {
        id: 1,
        text: "Hello, world!".to_string(),
        user_id: 42,
    };
    let display = format!("{}", msg);
    assert_eq!(display, "Message { id: 1, text: Hello, world!, user_id: 42 }");
}

#[test]
fn test_message_serialize_deserialize() {
    let msg = Message {
        id: 2,
        text: "Test serialization".to_string(),
        user_id: 99,
    };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(msg.id, deserialized.id);
}