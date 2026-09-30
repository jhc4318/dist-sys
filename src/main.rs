use std::io;

mod messages;
use serde_json::Deserializer;

use crate::messages::{Message, Payload};

fn handle_echo(msg: &Message, echo: &str) {
    let response = Message::new(
        msg.dst(),
        msg.src(),
        msg.id() + 1,
        Payload::EchoOk {
            in_reply_to: 1,
            echo: echo.to_string(),
        },
    );

    println!("{}", serde_json::to_string(&response).unwrap());
}

fn handle_init(msg: &Message) {
    let response = Message::new(
        msg.dst(),
        msg.src(),
        msg.id() + 1,
        Payload::InitOk { in_reply_to: 1 },
    );

    println!("{}", serde_json::to_string(&response).unwrap());
}

fn handle_msg(msg: &Message) {
    let msg_body = msg.payload();
    match msg_body {
        Payload::Echo { echo } => handle_echo(msg, echo),
        Payload::Init {
            node_id: _node_id,
            node_ids: _node_ids,
        } => handle_init(msg),
        _ => eprintln!("no response for {:?}", msg_body),
    }
}

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
    run()?;

    Ok(())
}
