use std::{env};
use log::{LevelFilter, info};
use env_logger::Builder;
use tokio_schedule::{every, Job};
use message::Message;

pub mod sender;
pub mod message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    let repeat = env::var("GATLING_MSG_SEC").expect("GATLING_MSG_SEC must be set").parse::<u32>().unwrap();
    let messages_nb = env::var("GATLING_MSG_NB").expect("GATLING_MSG_NB must be set").parse::<i32>().unwrap();

    info!("Gatling will repeat {} message(s) every {} seconds", messages_nb, repeat);
    let task = every(repeat).seconds().perform(gatling_execute);
    task.await;
     
    // Display results
    /*
    let messages = get_messages().await?;
    for message in &messages {
        info!("Message ID: {}, Text: {}, User: {}", message.id, message.text, message.user_id);
    }*/

    Ok(())
}

async fn gatling_execute() {
    info!("Executing Gatling task");
    
    // Generate messages
    let messages_nb = env::var("GATLING_MSG_NB").expect("GATLING_MSG_NB must be set").parse::<i32>().unwrap();
    let mut messages: Vec<Message> = Vec::new();
    for n in 0..messages_nb {
        let msg = Message {
            id: n,
            text: format!("Message from Gatling number {}", n),
            user_id: 1, // Assuming user_id 1 exists
        };
        messages.push(msg);
    }

    // Send messages asynchronously
    let send_type = env::var("SEND_TYPE").unwrap_or_else(|_| "wam".to_string());
    info!("Sending messages using method: {}", send_type);
    
    if send_type == "kafka" {
        sender::kafka::send_messages_to_kafka(messages).await.unwrap();
    } else {
        //send_messages(messages, sender::wamserver::send_to_wam_server).await.unwrap();
        let server_url = env::var("WAM_SERVER_URL").expect("WAM_SERVER_URL must be set");
        sender::wamserver::send_messages_to_wam_server(messages, &server_url).await.unwrap();
    }
}

/*pub async fn send_messages<F>(messages: Vec<Message>, f: fn(&Message) -> F) -> Result<(), Error> 
where
    F: Future<Output=Result<(), Error>>,
{
    let messages_sending: Vec<_> = messages
        .iter()
        .map(|msg| {
            f(msg)
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
}*/