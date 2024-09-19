//! coe is a library that (De-)serializes CoE packets.
//! The Protocol is owned by `Technische Alternative RT GmbH`.
//!
//! # Getting Started
//! You can send packets, by creating the Packet as required and then converting the packet into a
//! `Vec[u8]`.
//! ```ignore
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut test_packet = Packet::new();
//!     // send to CAN-ID 58, at offset 1 (shows up as 2 in the GUI)
//!     test_packet.try_push(Payload::new(58, 1, coe::COEValue::Analogue(AnalogueCOEValue::LiterPerPulse_Tens(123))))?;
//!
//!     let socket = UdpSocket::bind("0.0.0.0:34215").await?;
//!     let mut buf = [0_u8; 252];
//!     test_packet.try_serialize_into(&mut buf).expect("252 bytes is always large enough to fit a
//!     CoE Packet");
//!     // connect to the IP of your CMI
//!     socket.connect("192.168.1.123:5442").await?;
//!     socket.send(&buf).await?;
//!     Ok(())
//! }
//! ```
//!
//! # Feature Flags
//! The following feature flags are available:
//! - `std`: This is the default feature set.
//! - `serde`: This makes Packets, Paylods and Values Serializable with Serde.
//!
//! You can further opt-out of the default features with `default-features = false` your dependency listing for coe.
//! This makes `coe` depend only on [core], for use in no_alloc / no_std environments.
//! You can reenable the following feature flags
//! - `alloc`: This switches the implementation for a Packet from a fixed-size buffer to a Vec,
//! which is usually more memory-efficient. It also enables the [packets_from_payloads] function
//! and implements Display for Error types.
//! - Going from `alloc` to `std` implements [std::error::Error] on all Error types.
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

mod tests;

mod packet_common;

/// The Format a COE Value can have.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Format {
    Analogue,
    Digital,
}

/// All the Errors that can appear when parsing a COE packet.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ParseCOEError {
    /// The Node value is not allowed (1-62)
    NodeDisallowed(u8),
    /// The PDO is not allowed (0-63 on-wire == 1-64 in-GUI)
    PDOIndexDisallowed(u8),
    /// The Format (Analogue | Digital) and Unit are not compatible.
    /// Format: the format type expected
    /// u8: the Unit that was defined but is of the wrong Format type
    FormatAndUnitIncompatible(Format, u8),
    /// The Format is unknwon (neither Analogue nor Digital)
    FormatUnknown(u8),
    /// The Value hat an incorrect size.
    ValueSize(usize),
    /// A boolean value was expected due to the format, but not present.
    ValueNotBool([u8; 4]),
    /// The packet is shorter then the CoE header of 4 bytes
    PacketBelowHeaderLength,
    /// The CoE Protocol Version is not implemented.
    VersionNotImplemented(u8, u8),
    /// The packet size and payload length given are incompatible.
    PacketLengthInconsistent(u8, u8),
    /// The packet size given in the header is not the length of the packet actually found.
    PacketSizeConflictsWithHeader(u8, usize),
    /// Got a payload frame that is not 8 bytes long.
    PayloadFrameLengthIncorrect(usize),
}
#[cfg(feature = "alloc")]
impl alloc::fmt::Display for ParseCOEError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        match self {
            Self::NodeDisallowed(x) => {
                write!(f, "The Nodenumber must be in 1-62, but {} was supplied.", x)
            }
            Self::PDOIndexDisallowed(x) => {
                write!(f, "The PDO Index must be in 0-63, but {} was supplied.", x)
            }
            Self::FormatAndUnitIncompatible(format, unit) => match format {
                Format::Analogue => {
                    write!(
                        f,
                        "The unit with ID {unit} is not known as an analogue value in CoE."
                    )
                }
                Format::Digital => {
                    write!(
                        f,
                        "The unit with ID {unit} is not known as a digital value in CoE."
                    )
                }
            },
            Self::FormatUnknown(x) => {
                write!(f, "The Format with ID {} is not known.", &x)
            }
            Self::ValueSize(x) => {
                write!(
                    f,
                    "Slice containing COEValue was {x} bytes long. 4 expected."
                )
            }
            Self::ValueNotBool(x) => {
                write!(
                    f,
                    "Expected Value to be bool because of Format/Unit, but got {x:?}."
                )
            }
            Self::PacketBelowHeaderLength => {
                write!(f, "The packet is not at least 4 byte long.")
            }
            Self::VersionNotImplemented(major, minor) => {
                write!(f, "Version {major}.{minor} is not implemented.")
            }
            Self::PacketLengthInconsistent(size, length) => {
                write!(
                    f,
                    "The packet size ({size}) and payload length ({length}) are inconsistent."
                )
            }
            Self::PacketSizeConflictsWithHeader(header, actual) => {
                write!(
                    f,
                    "The packet size should be {header} but is actually {actual}."
                )
            }
            Self::PayloadFrameLengthIncorrect(actual) => {
                write!(f, "Got a payload frame of length {actual}. 8 expected.")
            }
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ParseCOEError {}

// NOTE: We only implement CoE v2.0 for now.
// Parsing a CoE packet of other versions will return an apropriate error.

#[cfg(feature = "alloc")]
mod packet_alloc;
#[cfg(feature = "alloc")]
pub use packet_alloc::Packet;

#[cfg(not(feature = "alloc"))]
mod packet_no_alloc;
#[cfg(not(feature = "alloc"))]
pub use packet_no_alloc::Packet;

/// Convert a slice of [Payload]s into (possibly multiple) [Packet]s.
///
/// This is infallible and always creates enough [Packet]s to pack all [Payload]s into.
///
/// This function is available only on the `alloc` feature flag.
#[cfg(feature = "alloc")]
pub fn packets_from_payloads(payloads: &[Payload]) -> Vec<Packet> {
    payloads
        .chunks(31)
        .map(|c| Packet::try_from_payloads(c).expect("Length should be satisfied by chunking"))
        .collect::<Vec<Packet>>()
}

/// The Version of COE protocol used.
#[derive(Hash, Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct COEVersion {
    /// The major CoE Version. Only 2 is supported.
    major: u8,
    /// The minor CoE Version. Only 0 is supported.
    minor: u8,
}
impl COEVersion {
    pub fn major(&self) -> u8 {
        self.major
    }
    pub fn minor(&self) -> u8 {
        self.minor
    }
}
impl TryFrom<(u8, u8)> for COEVersion {
    type Error = ParseCOEError;
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        // we only implement version 2.0 right now.
        if value.0 != 2 || value.1 != 0 {
            return Err(Self::Error::VersionNotImplemented(value.0, value.1));
        };
        Ok(COEVersion { major: 2, minor: 0 })
    }
}
#[cfg(feature = "alloc")]
impl alloc::fmt::Display for COEVersion {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// A single Payload that can be sent in a CoE packet.
///
/// This contains information about the destination (node and pdo_index) and the actual value
/// [COEValue].
///
/// NOTE: the pdo_index is offset by one to the representation in the GUI.
/// We store the on-wire format here, without the +1 offset present in the GUIs.
#[derive(Hash, Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Payload {
    /// The receiving CAN bus will create a virtual CAN node with this node number to send CAN
    /// messages onto the bus from.
    node: u8,
    /// The output index of the value on `node`
    pdo_index: u8,
    /// the Format field contains a u8 defining whether the value is analogue or digital
    /// The unit field then contains the unit ID - but each unit is uniquely either digital or
    /// analogue, so we do not need to store the format.
    value: COEValue,
}
/// Try to parse a `&[u8]` into a Payload.
impl TryFrom<&[u8]> for Payload {
    type Error = ParseCOEError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // bound check the node and pdo_index values:
        if value[0] == 0 || value[0] >= 63 {
            return Err(Self::Error::NodeDisallowed(value[0]));
        };
        if value[1] >= 64 {
            return Err(Self::Error::PDOIndexDisallowed(value[1]));
        };
        if value.len() != 8 {
            return Err(Self::Error::PayloadFrameLengthIncorrect(value.len()));
        };
        // read the format and unit value.
        // if they do not fit, return an Error
        // Otherwise, parse the actual value into COEValue
        match value[2] {
            0 => {
                let coe_value: DigitalCOEValue = (&value[3], &value[4..8]).try_into()?;
                Ok(Payload {
                    node: value[0],
                    pdo_index: value[1],
                    value: COEValue::Digital(coe_value),
                })
            }
            1 => {
                let coe_value: AnalogueCOEValue = (&value[3], &value[4..8]).try_into()?;
                Ok(Payload {
                    node: value[0],
                    pdo_index: value[1],
                    value: COEValue::Analogue(coe_value),
                })
            }
            _ => Err(Self::Error::FormatUnknown(value[2])),
        }
    }
}
impl core::default::Default for Payload {
    fn default() -> Payload {
        Payload {
            node: 1,
            pdo_index: 0,
            value: COEValue::Analogue(AnalogueCOEValue::Dimensionless(0)),
        }
    }
}
impl Payload {
    /// Create a new payload from the given destination and value.
    pub fn new(node: u8, pdo_index: u8, value: COEValue) -> Payload {
        Payload {
            node,
            pdo_index,
            value,
        }
    }

    /// Serialize this Payload into the given buffer
    /// the buffer MUST have length == 8
    /// the buffer MUST contain 0s in positions 5..8
    fn serialize_into(&self, buf: &mut [u8]) {
        // The only reason for this to not be satisfied is our internal code
        // passing the wrong buffer length, which should never happen.
        assert_eq!(buf.len(), 8);
        // write the node and pdo
        buf[0] = self.node;
        buf[1] = self.pdo_index;
        // now write the value
        self.value.serialize_into(&mut buf[2..8]);
    }
}

/// Any Value that is representable in COE.
#[derive(Hash, Debug, PartialEq, Copy, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum COEValue {
    /// An `analogue` Value.
    ///
    /// These are exactly the values which are represented as an i32 in the on-wire format.
    /// They are also called analogue in TAPPS etc.
    Analogue(AnalogueCOEValue),
    /// A `digital` Value.
    ///
    /// These are exactly the values which are represented as a bool in the on-wire format.
    /// They are also called digital in TAPPS etc.
    Digital(DigitalCOEValue),
}
impl COEValue {
    /// Serialize this COEValue into the given buffer
    /// the buffer MUST have length == 6
    /// the buffer MUST contain 0s in positions 3..6
    fn serialize_into(&self, buf: &mut [u8]) {
        assert_eq!(buf.len(), 6);
        match self {
            COEValue::Analogue(x) => {
                buf[0] = 1;
                x.serialize_into(&mut buf[1..6]);
            }
            COEValue::Digital(x) => {
                buf[0] = 0;
                x.serialize_into(&mut buf[1..6]);
            }
        };
    }
}

/// Convert a day and month into the internal format used in CoE.
///
/// NOTE: This does not check whether the month actually has that day.
/// (e.g. this will allow the 30th of february)
///
/// Returns `None` when `day` or `month` are out of bounds.
///
/// Example:
/// ```rust
/// use coe::{AnalogueCOEValue, to_day_of_month};
/// let val = to_day_of_month(17, 6);
/// assert_eq!(val, Some(AnalogueCOEValue::DayOfMonth(171)));
/// let val = to_day_of_month(58, 6);
/// assert_eq!(val, None);
/// let val = to_day_of_month(9, 14);
/// assert_eq!(val, None);
/// ```
pub fn to_day_of_month(day: u8, month: u8) -> Option<AnalogueCOEValue> {
    if day > 31 {
        return None;
    }
    if month > 12 {
        return None;
    }
    Some(AnalogueCOEValue::DayOfMonth(
        (day - 1) as i32 + (month - 1) as i32 * 31,
    ))
}

/// The Errors that can occur when parsing an integer as day of month
#[derive(Hash, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FromDayOfMonthError {
    /// The supplied AnalogueCOEValue was not DayOfMonth
    NotDayOfMonth,
    /// The value is not in bounds for regular FromDayOfMonth.
    /// Note that TA-Hardware always accepts the Value, but assignes useless values here instead.
    ValueOutOfBounds(i32),
}
#[cfg(feature = "alloc")]
impl alloc::fmt::Display for FromDayOfMonthError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        match self {
            Self::NotDayOfMonth => write!(f, "Value was not DayOfMonth"),
            Self::ValueOutOfBounds(x) => {
                write!(f, "The Value {x} cannot be parsed as a day, month pair.")
            }
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for FromDayOfMonthError {}

/// Convert the internal format for [AnalogueCOEValue::DayOfMonth] into two u8s containing day and month
///
/// Example:
/// ```rust
/// # use coe::{AnalogueCOEValue, from_day_of_month, FromDayOfMonthError};
/// let val = from_day_of_month(AnalogueCOEValue::DayOfMonth(173));
/// assert_eq!(val, Ok((19, 6)));
///
/// let val = from_day_of_month(AnalogueCOEValue::DayOfMonth(-1234));
/// assert_eq!(val, Err(FromDayOfMonthError::ValueOutOfBounds(-1234)));
///
/// let val = from_day_of_month(AnalogueCOEValue::DegreeKelvin_Tens(123));
/// assert_eq!(val, Err(FromDayOfMonthError::NotDayOfMonth));
/// ```
/// ```rust
/// use coe::{AnalogueCOEValue, from_day_of_month, to_day_of_month};
/// assert_eq!(from_day_of_month(to_day_of_month(9, 12).unwrap()), Ok((9, 12)));
/// ```
// Why is this not a method on AnalogueCOEValue?
// This conversion only makes sense when we have a DayOfMonth value.
// Keeping this function separate prevents IDEs from showing this method for all AnalogueCOEValues.
pub fn from_day_of_month(value: AnalogueCOEValue) -> Result<(u8, u8), FromDayOfMonthError> {
    match value {
        AnalogueCOEValue::DayOfMonth(x) => {
            if x < 0 || x > 30 + 31 * 11 {
                Err(FromDayOfMonthError::ValueOutOfBounds(x))
            } else {
                Ok((
                    (1 + x % 31).try_into().expect("Modulo 31 yields u8"),
                    (1 + x / 31).try_into().expect("Length was checked before"),
                ))
            }
        }
        _ => Err(FromDayOfMonthError::NotDayOfMonth),
    }
}

/// Convert a month and year into a [AnalogueCOEValue::DayOfMonth].
///
/// Returns `None` when `month` is out of bounds.
///
/// Example:
/// ```rust
/// # use coe::{AnalogueCOEValue, to_month_of_year};
/// let val = to_month_of_year(8, 1852);
/// assert_eq!(val, Some(AnalogueCOEValue::MonthOfYear(22231)));
///
/// let val = to_month_of_year(58, 6);
/// assert_eq!(val, None);
/// ```
pub fn to_month_of_year(month: u8, year: u16) -> Option<AnalogueCOEValue> {
    if month > 12 {
        return None;
    }
    Some(AnalogueCOEValue::MonthOfYear(
        (month - 1) as i32 + year as i32 * 12,
    ))
}

/// The Errors that can occur when parsing an integer as day of month
#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FromMonthOfYearError {
    /// The supplied AnalogueCOEValue was not MonthOfYear.
    NotMonthOfYear,
    /// The value is not in bounds for regular FromDayOfMonth.
    /// Note that TA-Hardware always accepts the Value, but assignes useless values here instead.
    ValueOutOfBounds(i32),
}
#[cfg(feature = "alloc")]
impl alloc::fmt::Display for FromMonthOfYearError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        match self {
            Self::NotMonthOfYear => write!(f, "Value was not MonthOfYear"),
            Self::ValueOutOfBounds(x) => {
                write!(f, "The Value {x} cannot be parsed as a month, year pair.")
            }
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for FromMonthOfYearError {}

/// Convert the internal format for [AnalogueCOEValue::MonthOfYear] into a `u8` and `u16` containing month and year
///
/// Example:
/// ```rust
/// # use coe::{AnalogueCOEValue, from_month_of_year, FromMonthOfYearError};
/// let val = from_month_of_year(AnalogueCOEValue::MonthOfYear(22231));
/// assert_eq!(val, Ok((8, 1852)));
///
/// let val = from_month_of_year(AnalogueCOEValue::MonthOfYear(13 * u16::MAX as i32));
/// assert_eq!(val, Err(FromMonthOfYearError::ValueOutOfBounds(13 * u16::MAX as i32)));
///
/// let val = from_month_of_year(AnalogueCOEValue::DegreeKelvin_Tens(123));
/// assert_eq!(val, Err(FromMonthOfYearError::NotMonthOfYear));
/// ```
/// ```rust
/// use coe::{AnalogueCOEValue, from_month_of_year, to_month_of_year};
/// assert_eq!(from_month_of_year(to_month_of_year(5, 325).unwrap()), Ok((5, 325)));
/// ```
// Why is this not a method on AnalogueCOEValue?
// This conversion only makes sense when we have a MonthOfYear value.
// Keeping this function separate prevents IDEs from showing this method for all AnalogueCOEValues.
pub fn from_month_of_year(value: AnalogueCOEValue) -> Result<(u8, u16), FromMonthOfYearError> {
    match value {
        AnalogueCOEValue::MonthOfYear(x) => {
            if x < 0 || x > 11 + 12 * u16::MAX as i32 {
                Err(FromMonthOfYearError::ValueOutOfBounds(x))
            } else {
                Ok((
                    (1 + x % 12).try_into().expect("Modulo 12 yields u8"),
                    (x / 12).try_into().expect("Length was checked before"),
                ))
            }
        }
        _ => Err(FromMonthOfYearError::NotMonthOfYear),
    }
}

/// All the different analogue values representable in CoE.
/// Ordering (and therefore numbering) is the one used internally in the CoE spec.
#[repr(u8)]
// We allow non_camel_case_types here, so that we can better separate the comma position from the
// actual content here (I think this is the cleaner naming scheme in this particular case)
#[allow(non_camel_case_types)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AnalogueCOEValue {
    Dimensionless(i32) = 0,
    DegreeCentigrade_Tens(i32) = 1,
    WattPerSquareMeter(i32) = 2,
    LiterPerHour(i32) = 3,
    Seconds(i32) = 4,
    Minutes(i32) = 5,
    LiterPerPulse_Tens(i32) = 6,
    DegreeKelvin_Tens(i32) = 7,
    Percent_Tens(i32) = 8,
    Colon(i32) = 9,
    KiloWatt_Hundreds(i32) = 10,
    KilowattHour_Tens(i32) = 11,
    MegawattHour(i32) = 12,
    Volt_Hundreds(i32) = 13,
    MilliAmpere_Tens(i32) = 14,
    Hours(i32) = 15,
    Days(i32) = 16,
    Pulses(i32) = 17,
    KiloOhm_Hundreds(i32) = 18,
    Liters(i32) = 19,
    KiloMetersPerHour(i32) = 20,
    Hertz_Hundreds(i32) = 21,
    LiterPerMinute(i32) = 22,
    Bar_Hundreds(i32) = 23,
    CoefficientOfPerformance_Hundreds(i32) = 24,
    KiloMeter(i32) = 25,
    Meter_Tens(i32) = 26,
    MilliMeter(i32) = 27,
    CubicMeter(i32) = 28,
    HertzPerKiloMeterPerHour_HundredThousands(i32) = 29,
    // Note: the documentation is incorrect here and lists this as Hz/km/s
    HertzPerMeterPerSecond_HundredThousands(i32) = 30,
    KilowattHourPerPulse_HundredThousands(i32) = 31,
    CubicMeterPerPulse_HundredThousands(i32) = 32,
    MilliMeterPerPulse_HundredThousands(i32) = 33,
    LiterPerPulse_HundredThousands(i32) = 34,
    LiterPerDay(i32) = 35,
    MetersPerSecond(i32) = 36,
    CubicMeterPerMinute(i32) = 37,
    CubicMeterPerHour(i32) = 38,
    CubicMeterPerDay(i32) = 39,
    MilliMeterPerMinute_Tens(i32) = 40,
    MilliMeterPerHour_Tens(i32) = 41,
    MilliMeterPerDay_Tens(i32) = 42,
    DegreeCentigradePlusRAS_Tens(i32) = 46,
    HeatingCircuitOpMode(i32) = 48,
    HeatingCircuitOpLevel(i32) = 49,
    CurrencyEuro_Hundreds(i32) = 50,
    CurrencyDollar_Hundreds(i32) = 51,
    AbsoluteHumidity_Tens(i32) = 52,
    PricePerUnit_HundredThousands(i32) = 53,
    Degree_Tens(i32) = 54,
    Blinds(i32) = 55,
    Degree_Millions(i32) = 56,
    Second_Tens(i32) = 57,
    Dimensionless_Tens(i32) = 58,
    BlindsPosition(i32) = 59,
    /// Time, in minutes, represented as HH:MM
    Time(i32) = 60,
    /// Day of month.
    /// `DayOfMonth(day - 1 + 31 * (month - 1))`
    /// corresponds to the day in month
    ///
    /// Consider using the helper functions [from_day_of_month] and [to_day_of_month] for parsing.
    DayOfMonth(i32) = 61,
    /// Date, as:
    /// days
    /// months
    /// years
    Date(u8, u8, u16) = 62,
    Ampere_Tens(i32) = 63,
    /// Month + Year.
    /// `MonthOfYear(year * 12 + month)` corresponds to month in year.
    ///
    /// Consider using the helper functions [from_month_of_year] and [to_month_of_year] for
    /// parsing.
    MonthOfYear(i32) = 64,
    Millibar_Tens(i32) = 65,
    Pascal(i32) = 66,
    CO2Content(i32) = 67,
    RawHex(i32) = 68,
    Watt(i32) = 69,
    Tonne_Hundreds(i32) = 70,
    KiloGram_Tens(i32) = 71,
    Gram_Tens(i32) = 72,
    CentiMeter_Tens(i32) = 73,
    ColourTemperature(i32) = 74,
    Lux_Tens(i32) = 75,
}

/// Given the Format and raw value in bytes, try to create the AnalogueCOEValue
impl TryFrom<(&u8, &[u8])> for AnalogueCOEValue {
    type Error = ParseCOEError;
    fn try_from(value: (&u8, &[u8])) -> Result<Self, Self::Error> {
        let raw_bytes: [u8; 4] = value
            .1
            .try_into()
            .map_err(|_| Self::Error::ValueSize(value.1.len()))?;
        let inner_value = i32::from_le_bytes(raw_bytes);
        match value.0 {
            0 => Ok(Self::Dimensionless(inner_value)),
            1 => Ok(Self::DegreeCentigrade_Tens(inner_value)),
            2 => Ok(Self::WattPerSquareMeter(inner_value)),
            3 => Ok(Self::LiterPerHour(inner_value)),
            4 => Ok(Self::Seconds(inner_value)),
            5 => Ok(Self::Minutes(inner_value)),
            6 => Ok(Self::LiterPerPulse_Tens(inner_value)),
            7 => Ok(Self::DegreeKelvin_Tens(inner_value)),
            8 => Ok(Self::Percent_Tens(inner_value)),
            9 => Ok(Self::Colon(inner_value)),
            10 => Ok(Self::KiloWatt_Hundreds(inner_value)),
            11 => Ok(Self::KilowattHour_Tens(inner_value)),
            12 => Ok(Self::MegawattHour(inner_value)),
            13 => Ok(Self::Volt_Hundreds(inner_value)),
            14 => Ok(Self::MilliAmpere_Tens(inner_value)),
            15 => Ok(Self::Hours(inner_value)),
            16 => Ok(Self::Days(inner_value)),
            17 => Ok(Self::Pulses(inner_value)),
            18 => Ok(Self::KiloOhm_Hundreds(inner_value)),
            19 => Ok(Self::Liters(inner_value)),
            20 => Ok(Self::KiloMetersPerHour(inner_value)),
            21 => Ok(Self::Hertz_Hundreds(inner_value)),
            22 => Ok(Self::LiterPerMinute(inner_value)),
            23 => Ok(Self::Bar_Hundreds(inner_value)),
            24 => Ok(Self::CoefficientOfPerformance_Hundreds(inner_value)),
            25 => Ok(Self::KiloMeter(inner_value)),
            26 => Ok(Self::Meter_Tens(inner_value)),
            27 => Ok(Self::MilliMeter(inner_value)),
            28 => Ok(Self::CubicMeter(inner_value)),
            29 => Ok(Self::HertzPerKiloMeterPerHour_HundredThousands(inner_value)),
            30 => Ok(Self::HertzPerMeterPerSecond_HundredThousands(inner_value)),
            31 => Ok(Self::KilowattHourPerPulse_HundredThousands(inner_value)),
            32 => Ok(Self::CubicMeterPerPulse_HundredThousands(inner_value)),
            33 => Ok(Self::MilliMeterPerPulse_HundredThousands(inner_value)),
            34 => Ok(Self::LiterPerPulse_HundredThousands(inner_value)),
            35 => Ok(Self::LiterPerDay(inner_value)),
            36 => Ok(Self::MetersPerSecond(inner_value)),
            37 => Ok(Self::CubicMeterPerMinute(inner_value)),
            38 => Ok(Self::CubicMeterPerHour(inner_value)),
            39 => Ok(Self::CubicMeterPerDay(inner_value)),
            40 => Ok(Self::MilliMeterPerMinute_Tens(inner_value)),
            41 => Ok(Self::MilliMeterPerHour_Tens(inner_value)),
            42 => Ok(Self::MilliMeterPerDay_Tens(inner_value)),
            46 => Ok(Self::DegreeCentigradePlusRAS_Tens(inner_value)),
            48 => Ok(Self::HeatingCircuitOpMode(inner_value)),
            49 => Ok(Self::HeatingCircuitOpLevel(inner_value)),
            50 => Ok(Self::CurrencyEuro_Hundreds(inner_value)),
            51 => Ok(Self::CurrencyDollar_Hundreds(inner_value)),
            52 => Ok(Self::AbsoluteHumidity_Tens(inner_value)),
            53 => Ok(Self::PricePerUnit_HundredThousands(inner_value)),
            54 => Ok(Self::Degree_Tens(inner_value)),
            55 => Ok(Self::Blinds(inner_value)),
            56 => Ok(Self::Degree_Millions(inner_value)),
            57 => Ok(Self::Second_Tens(inner_value)),
            58 => Ok(Self::Dimensionless_Tens(inner_value)),
            59 => Ok(Self::BlindsPosition(inner_value)),
            60 => Ok(Self::Time(inner_value)),
            61 => Ok(Self::DayOfMonth(inner_value)),
            62 => {
                let bytes = inner_value.to_le_bytes();
                let days = bytes[0];
                let months = bytes[1];
                let years: u16 = (bytes[2] as u16) + 256 * (bytes[3] as u16);
                Ok(Self::Date(days, months, years))
            }
            63 => Ok(Self::Ampere_Tens(inner_value)),
            64 => Ok(Self::MonthOfYear(inner_value)),
            65 => Ok(Self::Millibar_Tens(inner_value)),
            66 => Ok(Self::Pascal(inner_value)),
            67 => Ok(Self::CO2Content(inner_value)),
            68 => Ok(Self::RawHex(inner_value)),
            69 => Ok(Self::Watt(inner_value)),
            70 => Ok(Self::Tonne_Hundreds(inner_value)),
            71 => Ok(Self::KiloGram_Tens(inner_value)),
            72 => Ok(Self::Gram_Tens(inner_value)),
            73 => Ok(Self::CentiMeter_Tens(inner_value)),
            74 => Ok(Self::ColourTemperature(inner_value)),
            75 => Ok(Self::Lux_Tens(inner_value)),
            m => Err(Self::Error::FormatAndUnitIncompatible(Format::Analogue, *m)),
        }
    }
}
impl AnalogueCOEValue {
    /// Serialize this AnalogueCOEValue into the given buffer.
    /// The buffer MUST be of length == 5
    fn serialize_into(&self, buf: &mut [u8]) {
        assert_eq!(buf.len(), 5);
        match self {
            Self::Dimensionless(x) => {
                buf[0] = 0;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::DegreeCentigrade_Tens(x) => {
                buf[0] = 1;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::WattPerSquareMeter(x) => {
                buf[0] = 2;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::LiterPerHour(x) => {
                buf[0] = 3;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Seconds(x) => {
                buf[0] = 4;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Minutes(x) => {
                buf[0] = 5;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::LiterPerPulse_Tens(x) => {
                buf[0] = 6;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::DegreeKelvin_Tens(x) => {
                buf[0] = 7;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Percent_Tens(x) => {
                buf[0] = 8;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Colon(x) => {
                buf[0] = 9;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KiloWatt_Hundreds(x) => {
                buf[0] = 10;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KilowattHour_Tens(x) => {
                buf[0] = 11;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MegawattHour(x) => {
                buf[0] = 12;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Volt_Hundreds(x) => {
                buf[0] = 13;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliAmpere_Tens(x) => {
                buf[0] = 14;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Hours(x) => {
                buf[0] = 15;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Days(x) => {
                buf[0] = 16;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Pulses(x) => {
                buf[0] = 17;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KiloOhm_Hundreds(x) => {
                buf[0] = 18;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Liters(x) => {
                buf[0] = 19;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KiloMetersPerHour(x) => {
                buf[0] = 20;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Hertz_Hundreds(x) => {
                buf[0] = 21;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::LiterPerMinute(x) => {
                buf[0] = 22;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Bar_Hundreds(x) => {
                buf[0] = 23;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CoefficientOfPerformance_Hundreds(x) => {
                buf[0] = 24;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KiloMeter(x) => {
                buf[0] = 25;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Meter_Tens(x) => {
                buf[0] = 26;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliMeter(x) => {
                buf[0] = 27;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CubicMeter(x) => {
                buf[0] = 28;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::HertzPerKiloMeterPerHour_HundredThousands(x) => {
                buf[0] = 29;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::HertzPerMeterPerSecond_HundredThousands(x) => {
                buf[0] = 30;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KilowattHourPerPulse_HundredThousands(x) => {
                buf[0] = 31;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CubicMeterPerPulse_HundredThousands(x) => {
                buf[0] = 32;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliMeterPerPulse_HundredThousands(x) => {
                buf[0] = 33;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::LiterPerPulse_HundredThousands(x) => {
                buf[0] = 34;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::LiterPerDay(x) => {
                buf[0] = 35;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MetersPerSecond(x) => {
                buf[0] = 36;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CubicMeterPerMinute(x) => {
                buf[0] = 37;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CubicMeterPerHour(x) => {
                buf[0] = 38;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CubicMeterPerDay(x) => {
                buf[0] = 39;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliMeterPerMinute_Tens(x) => {
                buf[0] = 40;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliMeterPerHour_Tens(x) => {
                buf[0] = 41;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MilliMeterPerDay_Tens(x) => {
                buf[0] = 42;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::DegreeCentigradePlusRAS_Tens(x) => {
                buf[0] = 46;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::HeatingCircuitOpMode(x) => {
                buf[0] = 48;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::HeatingCircuitOpLevel(x) => {
                buf[0] = 49;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CurrencyEuro_Hundreds(x) => {
                buf[0] = 50;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CurrencyDollar_Hundreds(x) => {
                buf[0] = 51;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::AbsoluteHumidity_Tens(x) => {
                buf[0] = 52;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::PricePerUnit_HundredThousands(x) => {
                buf[0] = 53;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Degree_Tens(x) => {
                buf[0] = 54;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Blinds(x) => {
                buf[0] = 55;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Degree_Millions(x) => {
                buf[0] = 56;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Second_Tens(x) => {
                buf[0] = 57;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Dimensionless_Tens(x) => {
                buf[0] = 58;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::BlindsPosition(x) => {
                buf[0] = 59;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Time(x) => {
                buf[0] = 60;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::DayOfMonth(x) => {
                buf[0] = 61;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Date(days, months, years) => {
                buf[0] = 62;
                buf[1] = *days;
                buf[2] = *months;
                buf[3..5].copy_from_slice(&years.to_le_bytes());
            }
            Self::Ampere_Tens(x) => {
                buf[0] = 63;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::MonthOfYear(x) => {
                buf[0] = 64;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Millibar_Tens(x) => {
                buf[0] = 65;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Pascal(x) => {
                buf[0] = 66;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CO2Content(x) => {
                buf[0] = 67;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::RawHex(x) => {
                buf[0] = 68;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Watt(x) => {
                buf[0] = 69;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Tonne_Hundreds(x) => {
                buf[0] = 70;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::KiloGram_Tens(x) => {
                buf[0] = 71;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Gram_Tens(x) => {
                buf[0] = 72;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::CentiMeter_Tens(x) => {
                buf[0] = 73;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::ColourTemperature(x) => {
                buf[0] = 74;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
            Self::Lux_Tens(x) => {
                buf[0] = 75;
                buf[1..5].copy_from_slice(&x.to_le_bytes());
            }
        };
    }
}

/// Representation of all existing digital values representable in COE
#[repr(u8)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DigitalCOEValue {
    OnOff(bool) = 43,
    YesNo(bool) = 44,
    RASMode(bool) = 45,
    /// `true == Normal`,
    /// `false == AUS`
    Mixer(bool) = 47,
}

/// Given the Format and raw value in bytes, try to create the DigitalCOEValue
impl TryFrom<(&u8, &[u8])> for DigitalCOEValue {
    type Error = ParseCOEError;
    fn try_from(value: (&u8, &[u8])) -> Result<Self, Self::Error> {
        if value.1.len() != 4 {
            return Err(Self::Error::ValueSize(value.1.len()));
        };

        if value.1[3] != 0 || value.1[2] != 0 || value.1[1] != 0 {
            return Err(Self::Error::ValueNotBool(
                value
                    .1
                    .try_into()
                    .expect("I already asserted that value.1 has four elements."),
            ));
        };
        let inner_bool = match value.1[0] {
            0 => false,
            1 => true,
            _ => {
                return Err(Self::Error::ValueNotBool(
                    value
                        .1
                        .try_into()
                        .expect("I already asserted that value.1 has four elements."),
                ));
            }
        };
        match value.0 {
            43 => Ok(Self::OnOff(inner_bool)),
            44 => Ok(Self::YesNo(inner_bool)),
            45 => Ok(Self::RASMode(inner_bool)),
            47 => Ok(Self::Mixer(inner_bool)),
            m => Err(Self::Error::FormatAndUnitIncompatible(Format::Digital, *m)),
        }
    }
}
impl DigitalCOEValue {
    /// Serialize this DigitalCOEValue into the given buffer.
    /// The buffer MUST be of length == 5
    /// The buffer MUST contain 0 values in positions 2..5
    fn serialize_into(&self, buf: &mut [u8]) {
        assert_eq!(buf.len(), 5);
        match self {
            Self::OnOff(x) => {
                buf[0] = 43;
                buf[1] = *x as u8;
            }
            Self::YesNo(x) => {
                buf[0] = 44;
                buf[1] = *x as u8;
            }
            Self::RASMode(x) => {
                buf[0] = 45;
                buf[1] = *x as u8;
            }
            Self::Mixer(x) => {
                buf[0] = 47;
                buf[1] = *x as u8;
            }
        };
        // all other bits should be cleared
        // but this is already satisfied, because we expect a null buffer
    }
}
