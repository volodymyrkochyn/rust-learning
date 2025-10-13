use flexi_logger::{FileSpec, Logger};
use log::*;
use dirs;

fn init_logger() -> Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
    let mut dir = dirs::home_dir().unwrap();
    dir.push("logs");
    let logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().directory(dir))         // write logs to file
        .duplicate_to_stderr(flexi_logger::Duplicate::Warn)     // print warnings and errors also to the console
        .start()?;
    Ok(logger)
}

fn main() {
    init_logger().unwrap();

    info!("This goes to logs/log_0.log");
    warn!("This also goes to logs/log_0.log");
}
