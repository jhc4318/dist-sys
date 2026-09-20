use std::io;

mod messages;
use serde_json::Deserializer;

use crate::messages::{Echo, EchoOk, Message, MessageBody};

fn handle_echo(msg: &Message, echo: &str) {
    let response = Message::new(
        msg.get_dest(),
        msg.get_src(),
        MessageBody::EchoOk(EchoOk {
            msg_id: 1,
            in_reply_to: 1,
            echo: echo.to_string(),
        }),
    );

    println!("{:?}", serde_json::to_string(&response).unwrap());
}

fn handle_msg(msg: &Message) {
    let msg_body = msg.get_body();
    match msg_body {
        MessageBody::Echo(Echo {
            msg_id: _msg_id,
            echo,
        }) => handle_echo(msg, echo),
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
