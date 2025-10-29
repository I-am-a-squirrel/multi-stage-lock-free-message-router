use std::collections::HashMap;

pub(crate) type ProcessorsNumber = u8;
type ProcessingTimesNs = u16;

pub(crate) struct ProcessorConfig {
    count: ProcessorsNumber,
    processing_times_ns: HashMap<String, ProcessingTimesNs>,
}

fn start(number: ProcessorsNumber) -> () {
    for i in 0..number {
        tokio::spawn(future)
    }
}
