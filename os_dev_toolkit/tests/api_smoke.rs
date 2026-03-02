use os_dev_toolkit::buffer::{FixedStr, RingBuffer};
use os_dev_toolkit::fmt::{Addr, ByteFmt, HexSlice};
use os_dev_toolkit::log::{LogSink, RingLog};
use os_dev_toolkit::status::{KResult, Status};
use os_dev_toolkit::sync::{Once, SpinLock};

#[test]
fn ring_buffer_smoke() {
    let mut rb: RingBuffer<4> = RingBuffer::new();
    assert!(rb.is_empty());

    rb.push(1);
    rb.push(2);
    rb.push(3);
    rb.push(4);
    assert!(rb.is_full());

    // overwrite behavior
    rb.push(5);
    let (a, b) = rb.as_slices();
    let mut v = Vec::new();
    v.extend_from_slice(a);
    v.extend_from_slice(b);
    assert_eq!(v, vec![2, 3, 4, 5]);
}

#[test]
fn fixed_str_smoke() {
    let mut s: FixedStr<8> = FixedStr::new();
    assert!(s.try_push_str("ab").is_ok());
    assert_eq!(s.as_str(), "ab");
    assert!(s.try_push_str("cdefgh").is_ok());
    assert_eq!(s.as_str(), "abcdefgh");
    assert!(s.try_push_str("x").is_err());
}

fn returns_status(ok: bool) -> KResult<u32> {
    if ok {
        Ok(7)
    } else {
        Err(Status::InvalidArgument)
    }
}

#[test]
fn status_smoke() {
    assert_eq!(returns_status(true).unwrap(), 7);
    assert_eq!(returns_status(false).unwrap_err(), Status::InvalidArgument);
}

#[test]
fn ring_log_smoke() {
    let mut rl: RingLog<8> = RingLog::new();
    rl.write_str("abc");
    rl.write_str("defgh");
    let (a, b) = rl.as_slices();
    let mut v = Vec::new();
    v.extend_from_slice(a);
    v.extend_from_slice(b);
    // last 8 bytes of "abcdefgh"
    assert_eq!(v, b"abcdefgh".to_vec());
}

#[test]
fn fmt_helpers_smoke() {
    let bytes = [0u8, 0x41, 0x7f];
    assert_eq!(format!("{:?}", HexSlice(&bytes)), "[00 41 7f]");
    assert_eq!(format!("{}", ByteFmt(0)), "0 B");
    assert_eq!(format!("{}", ByteFmt(1024)), "1.000 KiB");
    assert_eq!(format!("{:x}", Addr(0x10usize)), "0x10");
}

#[test]
fn sync_smoke() {
    let lock = SpinLock::new(0u32);
    {
        let mut g = lock.lock();
        *g = 1;
    }
    assert_eq!(*lock.lock(), 1);

    static ONCE: Once<u32> = Once::new();
    let v = ONCE.call_once(|| 7);
    assert_eq!(*v, 7);
    assert_eq!(*ONCE.call_once(|| 9), 7);
}
