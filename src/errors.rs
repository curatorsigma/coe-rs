use alloc::string::String;

#[derive(Debug,PartialEq)]
pub enum ParseCOEError {
    NodeDisallowed(u8),
    PDOIndexDisallowed(u8),
    FormatAndUnitIncompatible(String),
    FormatUnknown(u8),
    ValueSize(usize),
    ValueNotBool([u8; 4]),
    PacketBelowHeaderLength,
    VersionNotImplemented(u8, u8),
    PacketLengthInconsistent(u8, u8),
    PacketSizeConflictsWithHeader(u8, usize),
    PayloadFrameLengthIncorrect(usize),
}
#[cfg(feature = "std")]
impl std::fmt::Display for ParseCOEError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NodeDisallowed(x) => { write!(f, "The Nodenumber must be in 1-62, but {} was supplied.", x) },
            Self::PDOIndexDisallowed(x) => { write!(f, "The PDO Index must be in 0-63, but {} was supplied.", x) },
            Self::FormatAndUnitIncompatible(x) => { write!(f, "FormatAndUnitIncompatible({x})") },
            Self::FormatUnknown(x) => { write!(f, "The Format with ID {} is not known.", &x) },
            Self::ValueSize(x) => { write!(f, "Slice containing COEValue was {x} bytes long. 4 expected.") },
            Self::ValueNotBool(x) => { write!(f, "Expected Value to be bool because of Format/Unit, but got {x:?}.") },
            Self::PacketBelowHeaderLength => { write!(f, "The packet is not at least 4 byte long.") },
            Self::VersionNotImplemented(major, minor) => { write!(f, "Version {major}.{minor} is not implemented.") },
            Self::PacketLengthInconsistent(size, length) => { write!(f, "The packet size ({size}) and payload length ({length}) are inconsistent.") },
            Self::PacketSizeConflictsWithHeader(header, actual) => { write!(f, "The packet size should be {header} but is actually {actual}.") },
            Self::PayloadFrameLengthIncorrect(actual) => { write!(f, "Got a payload from of length {actual}. 8 expected.") },
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ParseCOEError {}

#[derive(Debug,PartialEq)]
pub struct PacketMaxPayloadsExceeded {}
#[cfg(feature = "std")]
impl std::fmt::Display for PacketMaxPayloadsExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The maximal allowed number of payloads was exceeded.")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for PacketMaxPayloadsExceeded {}

