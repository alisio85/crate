use os_dev_toolkit::fmt::hexdump_to_sink;
use os_dev_toolkit::log::LogSink;

struct StdoutSink;
impl LogSink for StdoutSink {
    fn write_str(&mut self, s: &str) {
        print!("{}", s);
    }
}

fn main() {
    let data = [0u8, 1, 2, 3, 0xaa, 0xbb, 0xcc, 0xdd];
    let mut sink = StdoutSink;
    hexdump_to_sink(&data, &mut sink, 8);
}
