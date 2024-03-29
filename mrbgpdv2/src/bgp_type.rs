use crate::error::ConvertBytesToBgpMessageError;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct AutonomousSystemNumber(u16);

impl From<AutonomousSystemNumber> for u16 {
    fn from(as_number: AutonomousSystemNumber) -> u16 {
        as_number.0
    }
}

impl From<u16> for AutonomousSystemNumber {
    fn from(as_number: u16) -> Self {
        Self(as_number)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct HoldTime(u16);

impl From<HoldTime> for u16 {
    fn from(t: HoldTime) -> Self {
        t.0
    }
}

impl From<u16> for HoldTime {
    fn from(t: u16) -> Self {
        Self(t)
    }
}

impl Default for HoldTime {
    fn default() -> Self {
        HoldTime(0)
    }
}

impl HoldTime {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Version(u8);

impl From<Version> for u8 {
    fn from(v: Version) -> Self {
        v.0
    }
}

impl TryFrom<u8> for Version {
    type Error = ConvertBytesToBgpMessageError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if v <= 4 {
            Ok(Version(v))
        } else {
            Err(Self::Error::from(anyhow::anyhow!(
                "expect 1-4 for BGP's version but get {}", v
            )))
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version(4)
    }
}

impl Version {
    pub fn new() -> Self {
        Default::default()
    }
}