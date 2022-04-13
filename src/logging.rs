use std::io::Write;
use log::{Record, Level, Metadata};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(
                match record.level() {
                    Level::Error => Color::Red,
                    Level::Warn => Color::Yellow,
                    Level::Info => Color::Cyan,
                    _ => Color::White
                }
            ))).ok();
            writeln!(
                &mut stdout,
                "[{} - {}:{}] {}: {}",
                chrono::offset::Local::now().format("%I:%M:%S %p"),
                record.target(),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            ).ok();
            stdout.reset().ok();
        }
    }

    fn flush(&self) {}
}