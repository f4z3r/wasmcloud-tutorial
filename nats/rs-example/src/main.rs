use async_nats;
use dirs;
use serde::Deserialize;

use futures::StreamExt;
use std::str::from_utf8;
use tokio::fs;

const FILE: &'static str = ".config/nats/context/nats_development.json";

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // get login information from the existing config file
    let filename = dirs::home_dir().ok_or("failed to get home dir")?.join(FILE);
    let data = fs::read(filename).await?;
    let creds: Creds = serde_json::from_slice(&data).expect("JSON was not well-formatted");

    let options = async_nats::ConnectOptions::new().user_and_password(creds.user, creds.password);
    let client = async_nats::connect_with_options(creds.url, options).await?;

    // this will not be received, as the subscription is triggered after
    client.publish("greet.joe", "hello".into()).await?;

    // lets get three messages from the greet.* subjects
    let mut subscription = client.subscribe("greet.*").await?.take(3);

    // publish a couple of messages
    for subject in ["greet.sue", "greet.bob", "greet.pam"] {
        client.publish(subject, "hello".into()).await?;
    }

    // for three messages consumed, print information
    while let Some(message) = subscription.next().await {
        println!(
            "{:?} received on {:?}",
            from_utf8(&message.payload),
            &message.subject
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Creds {
    url: String,
    user: String,
    password: String,
}
