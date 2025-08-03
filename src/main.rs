use std::env;
use serde::{Serialize, Deserialize};
use reqwest::{Error, Client};
use log::{LevelFilter, info, debug};
use env_logger::Builder;
use futures::future::join_all;
use tokio_schedule::{every, Job};

#[derive(Serialize, Deserialize)]
struct Message {
    pub id: i32,
    pub text: String,
    pub user_id: i32,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ id: {}, text: {}, user_id: {} }}", self.id, self.text, self.user_id)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    let repeat = env::var("GATLING_MSG_SEC").expect("GATLING_MSG_SEC must be set").parse::<u32>().unwrap();
    info!("Gatling will repeat every {} seconds", repeat);
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
    let messages_sending: Vec<_> = messages
        .iter()
        .map(|msg| send_message(&msg.text, msg.user_id))
        .collect();

    let results = join_all(messages_sending).await;
    // Check for errors
    for result in results {
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn get_messages() -> Result<Vec<Message>, Error> {
    let server_url = env::var("WAM_SERVER_URL").expect("WAM_SERVER_URL must be set");
    let request_url = format!("{server_url}/message");
    info!("Calling request {}", request_url);

    // Make the HTTP get request
    let messages = Client::new()
        .get(request_url)
        .send()
        .await?
        .json::<Vec<Message>>()
        .await?;

    Ok(messages)
}

async fn send_message(text: &str, user_id: i32) -> Result<Message, Error> {
    let server_url = env::var("WAM_SERVER_URL").expect("WAM_SERVER_URL must be set");
    let request_url = format!("{server_url}/message");
    debug!("Sending message to {}", request_url);

    let message = Message {
        id: 0, // ID will be assigned by the server
        text: text.to_string(),
        user_id,
    };

    // Make the HTTP post request
    Client::new()
        .post(request_url)
        .json(&message)
        .send()
        .await?;

    info!("Message {} sent successfully", message);
    Ok(message)
}
