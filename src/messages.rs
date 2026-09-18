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
