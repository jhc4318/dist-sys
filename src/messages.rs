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

#[derive(Serialize, Debug, Deserialize)]
pub struct Init {
    pub msg_id: MsgId,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InitOk {
    pub in_reply_to: MsgId,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageBody {
    Echo(Echo),
    EchoOk(EchoOk),
    Init(Init),
    InitOk(InitOk),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_init_message() {
        let json = r#"
            {
                "src": "c1",
                "dest": "n1",
                "body": {
                    "type": "init",
                    "msg_id": 1,
                    "node_id": "n1",
                    "node_ids": ["n1", "n2"]
                }
            }
        "#;
        let msg = serde_json::from_str::<Message>(json).unwrap();

        assert_eq!(msg.get_src(), "c1");
        assert_eq!(msg.get_dest(), "n1");
        let MessageBody::Init(Init {
            msg_id,
            node_id,
            node_ids,
        }) = msg.get_body()
        else {
            panic!("did not parse as Init message");
        };
        assert_eq!(*msg_id, 1);
        assert_eq!(node_id, "n1");
        assert_eq!(*node_ids, vec!["n1", "n2"]);
    }

    #[test]
    fn parses_echo_message() {
        let json = r#"
            {
                "src": "c1",
                "dest": "n1",
                "body": {
                    "type": "echo",
                    "msg_id": 1,
                    "echo": "hi"
                }
            }
        "#;
        let msg = serde_json::from_str::<Message>(json).unwrap();

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
    fn rejects_unknown_message() {
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
