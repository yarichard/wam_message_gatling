use std::env;
use log::{LevelFilter, info};
use env_logger::Builder;

// Import modules from lib.rs
mod message;
mod sender;

// Import the shared core logic from lib.rs
use wam_message_gatling::GatlingCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    let repeat = env::var("GATLING_MSG_SEC").expect("GATLING_MSG_SEC must be set").parse::<u32>().unwrap();
    let messages_nb = env::var("GATLING_MSG_NB").expect("GATLING_MSG_NB must be set").parse::<i32>().unwrap();
    let server_url = env::var("WAM_SERVER_URL").expect("WAM_SERVER_URL must be set");

    info!("Gatling will send {} message(s) at {} msgs/sec to {}", messages_nb, repeat, server_url);
    
    // Create the shared core and execute
    let core = GatlingCore::new(messages_nb, repeat, server_url);
    
    // Execute using the shared logic
    let result = core.execute_standard().await?;
    info!("{}", result);
    
    Ok(())
}