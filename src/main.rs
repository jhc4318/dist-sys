use std::io::{self, BufRead};

mod messages;
use crate::messages::{Echo, EchoOk, Message, MessageBody};

fn handle_line(line: &str) {
    let Ok(msg) = serde_json::from_str::<Message>(line) else {
        eprintln!("could not parse {line} as a valid message");
        return;
    };
    eprintln!("received {msg:?}");

    let msg_body = msg.get_body();
    match msg_body {
        MessageBody::Echo(Echo {
            msg_id: _msg_id,
            echo,
        }) => handle_echo(&msg, echo),
        _ => eprintln!("no response for {:?}", msg_body),
    }
}

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

fn run() -> Result<(), io::Error> {
    eprintln!("Starting loop...");
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        handle_line(&line?);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()?;

    Ok(())
}
