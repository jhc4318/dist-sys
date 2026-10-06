use std::io;

mod messages;
use serde_json::Deserializer;
use tracing::info;

use crate::messages::{Message, Payload};

#[tracing::instrument]
fn handle_echo(msg: &Message, echo: &str) {
    let response = msg.reply(Payload::EchoOk {
        in_reply_to: msg.msg_id(),
        echo: echo.to_string(),
    });

    println!("{}", serde_json::to_string(&response).unwrap());
    info!("Responded to echo");
}

#[tracing::instrument]
fn handle_init(msg: &Message) {
    let response = msg.reply(Payload::InitOk {
        in_reply_to: msg.msg_id(),
    });
    println!("{}", serde_json::to_string(&response).unwrap());
    info!(response=?response, "Responded to init");
}

#[tracing::instrument]
fn handle_generate(msg: &Message) {
    let response = msg.reply(Payload::GenerateOk {
        in_reply_to: msg.msg_id(),
        id: uuid::Uuid::new_v4(),
    });
    println!("{}", serde_json::to_string(&response).unwrap());
    info!(response=?response, "Responded to generate");
}

#[tracing::instrument]
fn handle_msg(msg: &Message) {
    info!(msg=?msg, "Received a message");
    let msg_body = msg.payload();
    match msg_body {
        Payload::Echo { echo } => handle_echo(msg, echo),
        Payload::Init {
            node_id: _node_id,
            node_ids: _node_ids,
        } => handle_init(msg),
        Payload::Generate {} => handle_generate(msg),
        _ => info!("No response for {:?}", msg_body),
    }
}

#[tracing::instrument]
fn run() -> Result<(), io::Error> {
    eprintln!("Starting loop...");
    let stdin = io::stdin();

    let reader = Deserializer::from_reader(stdin.lock());

    for result in reader.into_iter::<Message>() {
        match result {
            Ok(msg) => handle_msg(&msg),
            Err(err) => eprintln!("could not parse response: {}", err),
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();
    run()?;

    Ok(())
}
