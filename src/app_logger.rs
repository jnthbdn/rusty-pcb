use std::sync::mpsc::Sender;

use log::{LevelFilter, SetLoggerError};

pub enum LogType {
    Info(String),
    Warning(String),
    Error(String),
}

pub struct AppLogger {
    sender: Sender<LogType>,
    module_name: String,
}

impl AppLogger {
    pub fn init(sender: Sender<LogType>, level: LevelFilter) -> Result<(), SetLoggerError> {
        let module = module_path!();
        let logger = Self {
            sender,
            module_name: module[0..module.find(":").unwrap_or(module.len())].to_string(),
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(())
    }
}

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        match &record.level() {
            log::Level::Error => {
                eprintln!(
                    "[{}] ({}) {}\n\r\t{}",
                    record.level(),
                    record.metadata().target(),
                    record.file().unwrap_or("???"),
                    record.args()
                )
            }
            log::Level::Warn | log::Level::Info | log::Level::Debug | log::Level::Trace => {
                println!(
                    "[{}] ({}) {}",
                    record.level(),
                    record.metadata().target(),
                    record.args()
                )
            }
        };

        if record.metadata().target().starts_with(&self.module_name) {
            let _ = match record.level() {
                log::Level::Error => self.sender.send(LogType::Error(format!(
                    "[{}:{}] {}",
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                ))),
                log::Level::Warn => self
                    .sender
                    .send(LogType::Warning(format!("{}", record.args()))),
                log::Level::Info | log::Level::Debug | log::Level::Trace => self
                    .sender
                    .send(LogType::Info(format!("{}", record.args()))),
            };
        }
    }

    fn flush(&self) {}
}
