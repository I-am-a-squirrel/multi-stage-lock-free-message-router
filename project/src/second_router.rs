use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected, Visitor},
};
use std::fmt;

use crate::{MessageNumber, MessageType, deserialize_message_type, strategies::StrategyCount};

pub(crate) mod init;

pub(crate) type ThreadsNumber = u8;

#[derive(Deserialize)]
pub(crate) struct Stage2Config {
    #[serde(deserialize_with = "deserialize_message_type")]
    msg_type: MessageType,
    strategy: StrategyCount,
    ordering_required: bool,
}
