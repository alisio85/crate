#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    InvalidArgument = 1,
    OutOfMemory = 2,
    NotSupported = 3,
    NotFound = 4,
    IoError = 5,
    Busy = 6,
    Timeout = 7,
    Unknown = 255,
}

pub type KResult<T> = core::result::Result<T, Status>;

impl Status {
    pub const fn is_ok(self) -> bool {
        matches!(self, Status::Ok)
    }
}

pub trait IntoStatus {
    fn into_status(self, err: Status) -> Status;
}

impl IntoStatus for bool {
    fn into_status(self, err: Status) -> Status {
        if self { Status::Ok } else { err }
    }
}

pub trait OptionIntoStatus<T> {
    fn ok_or_status(self, err: Status) -> KResult<T>;
}

impl<T> OptionIntoStatus<T> for Option<T> {
    fn ok_or_status(self, err: Status) -> KResult<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(err),
        }
    }
}
