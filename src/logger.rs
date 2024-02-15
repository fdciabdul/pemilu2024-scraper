use log::{Record, Level, Metadata, LevelFilter};
use colored::*; // Make sure this is added to Cargo.toml

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{} - {}", record.level(), record.args());
            match record.level() {
                Level::Error => println!("{}", message.red()),
                Level::Warn => println!("{}", message.yellow()),
                Level::Info => println!("{}", message.bright_cyan()),
                Level::Debug => println!("{}", message.blue()),
                Level::Trace => println!("{}", message.magenta()),
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}

static LOGGER: SimpleLogger = SimpleLogger;
