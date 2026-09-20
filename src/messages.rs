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
    Echo {
        msg_id: MsgId,
        echo: String,
    },
    EchoOk {
        msg_id: MsgId,
        in_reply_to: MsgId,
        echo: String,
    },
    Init {
        msg_id: MsgId,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: MsgId,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_init_msg() {
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
        let MessageBody::Init {
            msg_id,
            node_id,
            node_ids,
        } = msg.get_body()
        else {
            panic!("did not parse as Init message");
        };
        assert_eq!(*msg_id, 1);
        assert_eq!(node_id, "n1");
        assert_eq!(*node_ids, vec!["n1", "n2"]);
    }

    #[test]
    fn serializes_init_ok_msg() {
        let msg = Message::new("n1", "c1", MessageBody::InitOk { in_reply_to: 1 });

        let actual = serde_json::to_value(msg).unwrap();
        let expected = serde_json::json!({
            "src": "n1",
            "dest": "c1",
            "body": {
                "type": "init_ok",
                "in_reply_to": 1
            }
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn deserializes_echo_msg() {
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
        let MessageBody::Echo { msg_id, echo } = msg.get_body() else {
            panic!("did not parse as Echo message");
        };
        assert_eq!(*msg_id, 1);
        assert_eq!(echo, "hi");
    }

    #[test]
    fn serializes_echo_ok_msg() {
        let msg = Message::new(
            "n1",
            "c1",
            MessageBody::EchoOk {
                msg_id: 1,
                in_reply_to: 1,
                echo: "hello".to_string(),
            },
        );

        let actual = serde_json::to_value(msg).unwrap();
        let expected = serde_json::json!({
            "src": "n1",
            "dest": "c1",
            "body": {
                "type": "echo_ok",
                "msg_id": 1,
                "in_reply_to": 1,
                "echo": "hello"
            }
        });

        assert_eq!(actual, expected);
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
