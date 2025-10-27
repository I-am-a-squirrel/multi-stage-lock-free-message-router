use std::collections::HashMap;

pub(crate) type ProducersNumber = u8;
pub(crate) type MessagesPerSecond = u32;

pub(crate) struct ProducerConfig {
    count: ProducersNumber,
    messages_per_second: MessagesPerSecond,
    distribution: HashMap<String, f64>,
}

fn start(number: ProducersNumber) -> () {
    for i in 0..number {
        tokio::spawn(future)
    }
}
