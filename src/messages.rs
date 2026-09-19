use serde::{Deserialize, Serialize};

type MsgId = u16;

#[derive(Serialize, Debug, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

impl Message {
    pub fn new(src: &str, dest: &str, body: MessageBody) -> Self {
        Message {
            src: src.to_string(),
            dest: dest.to_string(),
            body,
        }
    }

    pub fn get_src(&self) -> &str {
        &self.src
    }

    pub fn get_dest(&self) -> &str {
        &self.dest
    }

    pub fn get_body(&self) -> &MessageBody {
        &self.body
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageBody {
    Echo(Echo),
    EchoOk(EchoOk),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Echo {
    pub msg_id: MsgId,
    pub echo: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct EchoOk {
    pub msg_id: MsgId,
    pub in_reply_to: MsgId,
    pub echo: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_message() {
        let msg = "{
            \"src\": \"c1\",
            \"dest\": \"n1\",
            \"body\": {
                \"type\": \"echo\",
                \"msg_id\": 1,
                \"echo\": \"hi\"
            }
        }";
        let msg = serde_json::from_str::<Message>(msg).unwrap();

        assert_eq!(msg.get_src(), "c1");
        assert_eq!(msg.get_dest(), "n1");
        let MessageBody::Echo(Echo { msg_id, echo }) = msg.get_body() else {
            panic!("did not parse as Echo message");
        };
        assert_eq!(*msg_id, 1);
        assert_eq!(echo, "hi");
    }

    #[test]
    #[should_panic]
    fn test_unknown_message() {
        let msg = "{
            \"src\": \"c1\",
            \"dest\": \"n1\",
            \"body\": {
                \"type\": \"unknown\",
            }
        }";

        serde_json::from_str::<Message>(msg).unwrap();
    }
}
