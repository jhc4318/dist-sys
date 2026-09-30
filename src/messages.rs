use serde::{Deserialize, Serialize};

type MsgId = usize;

#[derive(Serialize, Debug, Deserialize)]
pub struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Body {
    msg_id: MsgId,
    #[serde(flatten)]
    payload: Payload,
}

impl Message {
    pub fn new(src: &str, dest: &str, id: MsgId, payload: Payload) -> Self {
        Message {
            src: src.to_string(),
            dst: dest.to_string(),
            body: Body {
                msg_id: id,
                payload,
            },
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn dst(&self) -> &str {
        &self.dst
    }

    pub fn id(&self) -> MsgId {
        self.body.msg_id
    }

    pub fn payload(&self) -> &Payload {
        &self.body.payload
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        in_reply_to: MsgId,
        echo: String,
    },
    Init {
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

        assert_eq!(msg.src(), "c1");
        assert_eq!(msg.dst(), "n1");
        assert_eq!(msg.id(), 1);
        let Payload::Init { node_id, node_ids } = msg.payload() else {
            panic!("did not parse as Init message");
        };
        assert_eq!(node_id, "n1");
        assert_eq!(*node_ids, vec!["n1", "n2"]);
    }

    #[test]
    fn serializes_init_ok_msg() {
        let msg = Message::new("n1", "c1", 2, Payload::InitOk { in_reply_to: 1 });

        let actual = serde_json::to_value(msg).unwrap();
        let expected = serde_json::json!({
            "src": "n1",
            "dest": "c1",
            "body": {
                "type": "init_ok",
                "msg_id": 2,
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

        assert_eq!(msg.src(), "c1");
        assert_eq!(msg.dst(), "n1");
        assert_eq!(msg.id(), 1);
        let Payload::Echo { echo } = msg.payload() else {
            panic!("did not parse as Echo message");
        };
        assert_eq!(echo, "hi");
    }

    #[test]
    fn serializes_echo_ok_msg() {
        let msg = Message::new(
            "n1",
            "c1",
            2,
            Payload::EchoOk {
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
                "msg_id": 2,
                "in_reply_to": 1,
                "echo": "hello"
            }
        });

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn rejects_unknown_message() {
        let msg = r#"{
            "src": "c1",
            "dest": "n1",
            "body": {
                "type": "unknown",
                "msg_id": 1
            }
        }"#;

        serde_json::from_str::<Message>(msg).unwrap();
    }
}
