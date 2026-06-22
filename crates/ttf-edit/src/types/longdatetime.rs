use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LONGDATETIME([u8; 8]);

const EPOCH_NAIVE: NaiveDateTime =
    NaiveDate::from_ymd_opt(1904, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();

impl LONGDATETIME {
    pub const EPOCH: DateTime<Utc> = EPOCH_NAIVE.and_utc();

    pub const fn new(datetime: DateTime<Utc>) -> Self {
        let delta = datetime.naive_utc().signed_duration_since(EPOCH_NAIVE);
        Self(i64::to_be_bytes(delta.num_seconds()))
    }
    pub const fn to_datetime(&self) -> Option<DateTime<Utc>> {
        let secs = TimeDelta::seconds(i64::from_be_bytes(self.0));
        EPOCH_NAIVE.checked_add_signed(secs).map(const |dt| dt.and_utc())
    }
}

impl From<DateTime<Utc>> for LONGDATETIME {
    fn from(value: DateTime<Utc>) -> Self {
        Self::new(value)
    }
}
impl TryFrom<LONGDATETIME> for DateTime<Utc> {
    type Error = ();
    fn try_from(value: LONGDATETIME) -> Result<Self, Self::Error> {
        value.to_datetime().ok_or(())
    }
}

impl fmt::Debug for LONGDATETIME {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_datetime() {
            Some(datetime) => datetime.fmt(f),
            None => write!(f, "{:#X}", u64::from_be_bytes(self.0)),
        }
    }
}
