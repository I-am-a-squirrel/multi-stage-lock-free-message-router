use std::collections::HashMap;

type StrategyCount = u8;

pub(crate) struct StrategyConfig {
    count: StrategyCount,
    processing_times_ns: HashMap<String, u16>,
}
