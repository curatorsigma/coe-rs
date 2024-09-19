//! Implement [Packet] with the alloc feature enabled

use self::packet_common::PacketIterator;

use super::*;

/// A COE Packet
///
/// This models every possible Packet that can be send via CoE.
/// It consists mostly of [Payload]s, which can be added by different means.
/// Note that a Packet can at most contain 31 Payloads, so that all methods adding new Payloads can
/// fail.
// Note: we enforce and assume that `payload.len()` never exceeds 31.
// This is required, because the packet contains its own size (in bytes) in a field containing a
// u8, so no more then 255 (`u8::MAX`) bytes may ever be contained in a packets full representation.
// The packet on wire contains 4 bytes of headers, leaving us with 251 usable bytes. A payload
// length of 8 byte per payload yields 31 full payloads that fit in the max packet length.
#[derive(Hash, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Packet {
    /// CoE Version used. Currently, only 2.0 is supported.
    version: COEVersion,
    /// The actual payloads.
    pub(crate) payloads: Vec<Payload>,
}
impl TryFrom<&[u8]> for Packet {
    type Error = ParseCOEError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // the header must be four byte long
        if value.len() < 4 {
            return Err(Self::Error::PacketBelowHeaderLength);
        };
        // parse the version number from the first two bytes
        let version: COEVersion = (value[0], value[1]).try_into()?;
        // assert that packet length and payload length are consistent
        if value[2] != 4 + 8 * value[3] {
            return Err(Self::Error::PacketLengthInconsistent(value[2], value[3]));
        };
        // we are now certain that the header is correctly formed.
        // Assert that the packet actually has the correct size as given in the header.
        if value.len() != value[2].into() {
            return Err(Self::Error::PacketSizeConflictsWithHeader(
                value[2],
                value.len(),
            ));
        };
        // The packet has the correct length. We can chunk it and parse each value independently
        // without additional checks for buffer overrun
        let mut payloads: Vec<Payload> = vec![];
        for payload_nr in 0..value[3] {
            // each payload is exactly 8 bytes long - +4 is the header offset
            payloads.push(
                value[(payload_nr * 8 + 4) as usize..=(payload_nr * 8 + 11) as usize].try_into()?,
            );
        }
        Ok(Packet { version, payloads })
    }
}
impl From<Packet> for Vec<u8> {
    /// Serialize a packet into `Vec<u8>`
    ///
    /// This is guaranteed to succeed since a Packet can never have more then 31 payloads, such
    /// that the resulting serialization will always be at most 255 bytes long.
    fn from(value: Packet) -> Self {
        let mut res = vec![0_u8; 4 + value.payloads.len() * 8];
        // Packet always successfully serializes, since we set the size correctly
        value.try_serialize_into(&mut res).unwrap();
        return res;
    }
}
impl IntoIterator for Packet {
    type Item = Payload;
    type IntoIter = alloc::vec::IntoIter<Payload>;
    fn into_iter(self) -> Self::IntoIter {
        self.payloads.into_iter()
    }
}
impl<'a> IntoIterator for &'a Packet {
    type Item = &'a Payload;
    type IntoIter = core::slice::Iter<'a, Payload>;
    fn into_iter(self) -> Self::IntoIter {
        self.payloads.iter()
    }
}
impl<'a> IntoIterator for &'a mut Packet {
    type Item = &'a mut Payload;
    type IntoIter = core::slice::IterMut<'a, Payload>;
    fn into_iter(self) -> Self::IntoIter {
        self.payloads.iter_mut()
    }
}
impl Packet {
    /// Create a packet without payloads
    pub fn new() -> Packet {
        Packet {
            version: COEVersion { major: 2, minor: 0 },
            payloads: vec![],
        }
    }

    /// The number of payloads in this packet.
    pub fn len(&self) -> usize {
        self.payloads.len()
    }

    /// The size this packet would have on-wire in bytes.
    ///
    /// For example, this code is safe:
    /// ```
    /// # use coe::Packet;
    /// let packet = Packet::new();
    /// let mut buf = vec![0_u8; packet.wire_size()];
    /// packet.try_serialize_into(&mut buf).expect("I set the wire-size correctly.");
    /// ```
    pub fn wire_size(&self) -> usize {
        4 + self.len() * 8
    }

    /// Get the COE Version of this Packet.
    pub fn version(&self) -> COEVersion {
        self.version
    }

    /// Get the payloads of this Packet by immutable reference
    pub fn iter(&self) -> PacketIterator {
        PacketIterator::new(self)
    }

    /// Get the payloads of this Packet by mutable reference.
    pub fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, Payload> {
        self.payloads.iter_mut()
    }

    /// Create a [Packet] with [Payload]s. Fails if more then 31 payloads are given.
    pub fn try_from_payloads(payloads: &[Payload]) -> Option<Packet> {
        let mut p = Packet::new();
        p.try_append_from_slice(payloads)?;
        Some(p)
    }

    /// Try to append a [Payload] to a [Packet]
    ///
    /// Fails if the final packet size would exceed 255 bytes (31 payloads).
    /// On failure, the packet was left unmodified.
    pub fn try_push(&mut self, payload: Payload) -> Option<()> {
        if (self.payloads.len() + 1) * 8 + 4 >= u8::MAX as usize {
            return None;
        };
        self.payloads.push(payload);
        Some(())
    }

    /// Try to append all the given [Payload]s to a [Packet]
    ///
    /// Fails if the final packet size would exceed 255 bytes (31 payloads).
    /// On failure, the packet was left unmodified.
    pub fn try_append_from_slice(&mut self, payloads: &[Payload]) -> Option<()> {
        if (self.payloads.len() + payloads.len()) * 8 + 4 >= u8::MAX as usize {
            return None;
        };
        self.payloads.extend_from_slice(payloads);
        Some(())
    }

    /// Serialize this Packet into a `&[u8]` which can be sent on-the-wire.
    ///
    /// This can fail if buf is to small, in which case `None` is returned.
    /// Otherwise, return the amount of bytes written into `buf`.
    pub fn try_serialize_into(&self, buf: &mut [u8]) -> Option<usize> {
        if buf.len() < 4 + self.payloads.len() as usize * 8 {
            return None;
        };
        // the HEADER
        buf[0] = self.version.major;
        buf[1] = self.version.minor;
        buf[2] = 4 + self.payloads.len() as u8 * 8;
        buf[3] = self.payloads.len() as u8;

        // the PAYLOAD
        // now set each individual payload
        for (index, payload) in self.payloads.iter().enumerate() {
            payload.serialize_into(&mut buf[4 + index * 8..=11 + index * 8]);
        }
        Some(4 + self.payloads.len() as usize * 8)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_packet_success() {
        let raw_bytes = [
            2, 0, 20, 2, 3, 0, 1, 1, 95, 0, 0, 0, 3, 0, 0, 43, 1, 0, 0, 0,
        ];
        let packet: crate::Packet = raw_bytes[0..20]
            .try_into()
            .expect("This Packet is parsable.");
        assert_eq!(
            packet,
            crate::Packet {
                version: crate::COEVersion { major: 2, minor: 0 },
                payloads: alloc::vec![
                    crate::Payload {
                        node: 3,
                        pdo_index: 0,
                        value: crate::COEValue::Analogue(
                            crate::AnalogueCOEValue::DegreeCentigrade_Tens(95)
                        )
                    },
                    crate::Payload {
                        node: 3,
                        pdo_index: 0,
                        value: crate::COEValue::Digital(crate::DigitalCOEValue::OnOff(true))
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_packet_below_header_length() {
        let raw_bytes = [2, 0, 20];
        let err: crate::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..3])
            .expect_err("This Packet is not parsable.");
        assert_eq!(err, crate::ParseCOEError::PacketBelowHeaderLength);
    }

    #[test]
    fn parse_packet_packet_length_inconsistent() {
        let raw_bytes = [
            2, 0, 21, 2, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1,
        ];
        let err: crate::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20])
            .expect_err("This Packet is not parsable");
        assert_eq!(
            err,
            crate::ParseCOEError::PacketLengthInconsistent(21_u8, 2_u8)
        );
    }

    #[test]
    fn parse_packet_packet_length_inconsistent_2() {
        let raw_bytes = [
            2, 0, 20, 3, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1,
        ];
        let err: crate::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20])
            .expect_err("This Packet is not parsable");
        assert_eq!(
            err,
            crate::ParseCOEError::PacketLengthInconsistent(20_u8, 3_u8)
        );
    }

    #[test]
    fn parse_packet_packet_size_conflicts_with_header() {
        let raw_bytes = [
            2, 0, 12, 1, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1,
        ];
        let err: crate::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20])
            .expect_err("This Packet is not parsable");
        assert_eq!(
            err,
            crate::ParseCOEError::PacketSizeConflictsWithHeader(12, 20)
        );
    }
}
