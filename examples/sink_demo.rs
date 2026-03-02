use os_dev_toolkit::log::{Level, LogSink, Logger};

struct StdoutSink;

impl LogSink for StdoutSink {
    fn write_str(&mut self, s: &str) {
        print!("{}", s);
    }
}

fn main() {
    let mut sink = StdoutSink;
    let mut logger = Logger::new(&mut sink, Level::Debug);

    os_dev_toolkit::kinfo!(logger, "info: {}", 1);
    os_dev_toolkit::kdebug!(logger, "debug: {:x}", 0x2a);
}
