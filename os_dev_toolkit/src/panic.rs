use core::fmt;
use core::fmt::Write;

use crate::log::LogSink;

pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

pub fn panic_to_sink(info: &core::panic::PanicInfo<'_>, sink: &mut dyn LogSink) -> ! {
    struct Adapter<'a>(&'a mut dyn LogSink);
    impl fmt::Write for Adapter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s);
            Ok(())
        }
    }

    let mut a = Adapter(sink);
    let _ = writeln!(a, "PANIC: {info}");
    halt()
}
