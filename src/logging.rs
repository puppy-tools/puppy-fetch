use log::LevelFilter;
use std::io::Write;

// initializes a logger with no formatting
pub fn plain_logger(level: LevelFilter) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::new()
        .format(|buf, record| { write!(buf, "{}", record.args())})
        .filter_level(level)
        .try_init()
}

pub mod macros {
    /* 
        by default env_logger includes a newline character,
        in this context it's not desirable since the carriage return is used in multiple places,
        with a newline this breaks print formatting for things like:

        - sideband progress
        - transfer progress
            - received objects
            - resolved deltas

        this set of macros allows for a more dev-friendly way to control formatting in an expected way.
    */

    #[macro_export] macro_rules! error { ($($arg:tt)*) => { log::error!("\x1b[31m{}\x1b[0m", format!($($arg)*)) }; }
    #[macro_export] macro_rules! warn { ($($arg:tt)*) => { log::warn!("\x1b[33m{}\x1b[0m", format!($($arg)*)) }; }
    #[macro_export] macro_rules! info { ($($arg:tt)*) => { log::info!("{}", format!($($arg)*)) }; }
    #[macro_export] macro_rules! debug { ($($arg:tt)*) => { log::debug!("\x1b[36m{}\x1b[0m", format!($($arg)*)) }; }
    #[macro_export] macro_rules! trace { ($($arg:tt)*) => { log::trace!("\x1b[2m{}\x1b[22m", format!($($arg)*)) }; }

    #[macro_export] macro_rules! errorln { ($($arg:tt)*) => { log::error!("\x1b[31m{}\x1b[0m\n", format!($($arg)*)) }; }
    #[macro_export] macro_rules! warnln { ($($arg:tt)*) => { log::warn!("\x1b[33m{}\x1b[0m\n", format!($($arg)*)) }; }
    #[macro_export] macro_rules! infoln { ($($arg:tt)*) => { log::info!("{}\n", format!($($arg)*)) }; }
    #[macro_export] macro_rules! debugln { ($($arg:tt)*) => { log::debug!("\x1b[36m{}\x1b[0m\n", format!($($arg)*)) }; }
    #[macro_export] macro_rules! traceln { ($($arg:tt)*) => { log::trace!("\x1b[2m{}\x1b[22m\n", format!($($arg)*)) }; }
}
