use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Chat {
        data: Bytes,
        sender: u128,
        chat_room: u128,
    },
    Auth {
        login: Bytes,
        password: Bytes,
    },
    Subscribe {
        chat_room: u128,
        sender: u128,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    user_id: u128,
}

pub fn serialize_msg(msg: &Msg) -> (u64, Bytes) {
    let msg = bincode::serialize(msg).expect("something really fucked up");
    (msg.len() as u64, Bytes::from(msg))
}

pub fn deserialize_msg(data: &[u8]) -> Msg {
    bincode::deserialize(&data).expect("something really fucked up")
}

impl Session {
    pub fn create_chat_msg(self, s: String, chat_room: u128) -> Msg {
        Msg::Chat {
            data: Bytes::from(s),
            sender: self.user_id,
            chat_room,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_msg_serialization() {
        let msg_text = String::from("dsaldjsalkj");
        let chat_room = 1;
        let session = Session { user_id: 1 };

        let chat_message = session.create_chat_msg(msg_text, chat_room);
        let (size, data) = serialize_msg(&chat_message);
        let retrieved_msg = deserialize_msg(&data[..]);

        // assert_eq!(chat_message, retrieved_msg);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
