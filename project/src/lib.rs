use chrono::Duration;
use num_enum::TryFromPrimitive;

use crate::{
    primary_router::Stage1Config, processors::ProcessorConfig, producers::ProducerConfig,
    second_router::Stage2Config, strategies::StrategyConfig,
};

mod primary_router;
mod processors;
mod producers;
mod second_router;
mod strategies;

type MessageNumber = u8;

pub fn deserialize_message_type<'de, D>(deserializer: D) -> Result<MessageType, D::Error>
where
    D: Deserializer<'de>,
{
    struct CustomString;

    impl<'de> Visitor<'de> for CustomString {
        type Value = MessageType;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expecting msg_type_0 format")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let int_str = v
                .strip_prefix("msg_type_")
                .ok_or("This field starts with not msg_type_")
                .map_err(|_| de::Error::invalid_value(Unexpected::Str(v), &self))?;
            let enum_int = int_str
                .parse::<MessageNumber>()
                .map_err(|_| de::Error::invalid_value(Unexpected::Str(v), &self))?;
            Ok(MessageType::try_from(enum_int).unwrap())
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_any(CustomString)
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum MessageType {
    MessageType0,
    MessageType1,
    MessageType2,
    MessageType3,
    MessageType4,
    MessageType5,
    MessageType6,
    MessageType7,
}

pub(crate) struct Config {
    scenario: String,
    duration: Duration,
    producers: ProducerConfig,
    processors: ProcessorConfig,
    strategies: StrategyConfig,
    stage1_rules: Stage1Config,
    stage2_rules: Stage2Config,
}
