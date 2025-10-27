use chrono::Duration;

use crate::{processors::ProcessorConfig, producers::ProducerConfig, strategies::StrategyConfig};

mod primary_router;
mod processors;
mod producers;
mod second_router;
mod strategies;

pub(crate) struct Config {
    scenario: String,
    duration: Duration,
    producers: ProducerConfig,
    processors: ProcessorConfig,
    strategies: StrategyConfig,
    stage1_rules: ,
    stage2_rules: ,
}

