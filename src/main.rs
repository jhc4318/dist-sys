use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

type MsgId = u16;

#[derive(Serialize, Debug, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageBody {
    Echo {
        msg_id: MsgId,
        echo: String,
    },
    EchoOk {
        msg_id: MsgId,
        in_reply_to: MsgId,
        echo: String,
    },
}

fn run() -> Result<(), io::Error> {
    eprintln!("Starting loop...");
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let msg: Message = serde_json::from_str(&line?)?;
        println!("{:?}", msg);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()?;

    Ok(())
}
