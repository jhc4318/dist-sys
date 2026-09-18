use std::io::{self, BufRead};

mod messages;
use crate::messages::Message;

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
