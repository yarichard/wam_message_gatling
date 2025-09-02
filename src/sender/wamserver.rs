use futures::future::join_all;
use reqwest::{Error, Client};
use log::{info, debug};
use crate::message::Message;

pub async fn send_messages_to_wam_server(messages: Vec<Message>, server_url: &String) -> Result<(), Error> {
    let messages_sending: Vec<_> = messages
        .iter()
        .map(|msg| {
            send_to_wam_server(msg, server_url)
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

pub async fn send_to_wam_server(message: &Message, server_url: &String) -> Result<(), Error> {
    let request_url = format!("{server_url}/message");
    debug!("Sending message to {}", request_url);

    // Make the HTTP post request
    Client::new()
        .post(request_url)
        .json(&message)
        .send()
        .await?;

    info!("Message {} sent successfully", message);
    Ok(())
}

/* 
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
}*/