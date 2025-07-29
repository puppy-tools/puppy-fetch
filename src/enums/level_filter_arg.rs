use clap::ValueEnum;
use log::LevelFilter;

/* 
    This enum may *seem* redundant, 
    however it exists to take advantage of clap's derive features,
    more specifically the clap::ValueEnum trait.

    clap::ValueEnum cannot be implemented for LevelFilter because of Rust's orphan rule:
    https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules

    TODO: Potentially move to tuple struct ? 
*/

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum LevelFilterArg {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

impl From<LevelFilterArg> for LevelFilter {
    fn from(value: LevelFilterArg) -> Self {
        match value {
            LevelFilterArg::Off => LevelFilter::Off,
            LevelFilterArg::Error => LevelFilter::Error,
            LevelFilterArg::Warn => LevelFilter::Warn,
            LevelFilterArg::Info => LevelFilter::Info,
            LevelFilterArg::Debug => LevelFilter::Debug,
            LevelFilterArg::Trace => LevelFilter::Trace,
        }
    }
}


