use serde::{Deserialize, Serialize};

type MsgId = u16;

#[derive(Serialize, Debug, Deserialize)]
pub struct Message {
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

        assert_eq!(msg.src, "c1");
        assert_eq!(msg.dest, "n1");
        let MessageBody::Echo { msg_id, echo } = msg.body else {
            panic!("did not parse as Echo message");
        };
        assert_eq!(msg_id, 1);
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
