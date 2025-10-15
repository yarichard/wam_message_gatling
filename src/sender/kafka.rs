/*use kafka::producer::{Producer, Record, RequiredAcks};
use futures::future::join_all;
use reqwest::{Error};
use std::{env, time::Duration};
use log::{info, error};
use crate::message::Message;

pub async fn send_messages_to_kafka(messages: Vec<Message>) -> Result<(), Error> {     
    let messages_sending: Vec<_> = messages
        .iter()
        .map(|msg| {
            send_to_kafka(msg)
        })
        .collect();

    let results = join_all(messages_sending).await;
    // Check for errors
    for result in results {
        match result {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}


pub async fn send_to_kafka(message: &Message) -> Result<(), Error>{
    let host = env::var("KAFKA_URL").expect("KAFKA_URL must be set");
    let topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set");
    let mut producer = Producer::from_hosts(vec![host.to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();
            
    let buffer = serde_json::to_string(message).unwrap();

    let result = producer
        .send(&Record::from_value(topic.as_str(), buffer.as_bytes()));
    
    match result {
        Ok(_) => {
            info!("Message {} sent to Kafka topic {}", message, topic);
        },
        Err(e) => {
            error!("Failed to send message to Kafka: {}", e);
        }
    }

    Ok(())
}*/