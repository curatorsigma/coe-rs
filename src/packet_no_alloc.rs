//! Implement [Packet] with the alloc feature disabled

use super::*;

use self::packet_common::PacketIterator;

/// A COE Packet
///
/// This models every possible Packet that can be send via CoE.
/// It consists mostly of [Payload]s, which can be added by different means.
/// Note that a Packet can at most contain 31 Payloads, so that all methods adding new Payloads can
/// fail.
// Note: we enforce and assume that `payload_length` never exceeds 31.
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
    pub(crate) payloads: [Payload; 31],
    /// The amount of payloads actually used
    /// The remaining payloads are defaulted, but SHOULD NOT be read, because they contain no
    /// semantic
    payload_length: u8,
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
        let mut payloads = [Payload::default(); 31];
        let mut idx = 0;
        for payload_nr in 0..value[3] {
            // each payload is exactly 8 bytes long - +4 is the header offset
            let payload: Payload =
                value[(payload_nr * 8 + 4) as usize..=(payload_nr * 8 + 11) as usize].try_into()?;
            payloads[idx] = payload;
            idx += 1;
        }
        // because we have checked that the packet length is consistent with the payload number
        // and that the actual packet size is as advertised in the header, we now that
        // idx can never be larger then 31, so this cast never looses information
        Ok(Packet {
            version,
            payloads,
            payload_length: idx as u8,
        })
    }
}
// the largest well-formed packet can be 4 + 31 * 8 bytes long (header + 31 Payloads of 8 byte
// each)
impl From<Packet> for [u8; 4 + 8 * 31] {
    /// Serialize a packet into `Vec<u8>`
    ///
    /// This is guaranteed to succeed since a Packet can never have more then 31 payloads, such
    /// that the resulting serialization will always be at most 255 bytes long.
    fn from(value: Packet) -> Self {
        let mut res = [0_u8; 4 + 8 * 31];
        // Packet always successfully serializes into a 252-byte array.
        value.try_serialize_into(&mut res).unwrap();
        res
    }
}
pub struct PacketNoAllocIntoIter {
    packet: Packet,
    idx: usize,
}
impl Iterator for PacketNoAllocIntoIter {
    type Item = Payload;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.packet.len() {
            let res = Some(self.packet.payloads[self.idx]);
            self.idx += 1;
            res
        } else {
            None
        }
    }
}
impl IntoIterator for Packet {
    type Item = Payload;
    type IntoIter = PacketNoAllocIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        PacketNoAllocIntoIter {
            packet: self,
            idx: 0,
        }
    }
}
impl<'a> IntoIterator for &'a Packet {
    type Item = &'a Payload;
    type IntoIter = core::slice::Iter<'a, Payload>;
    fn into_iter(self) -> Self::IntoIter {
        self.payloads[0..self.payload_length as usize].iter()
    }
}
impl<'a> IntoIterator for &'a mut Packet {
    type Item = &'a mut Payload;
    type IntoIter = core::slice::IterMut<'a, Payload>;
    fn into_iter(self) -> Self::IntoIter {
        self.payloads[0..self.payload_length as usize].iter_mut()
    }
}
impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}
impl Packet {
    /// Create a packet without payloads
    pub fn new() -> Packet {
        Packet {
            version: COEVersion { major: 2, minor: 0 },
            payloads: [Payload::default(); 31],
            payload_length: 0,
        }
    }

    /// The number of payloads in this packet.
    pub fn len(&self) -> usize {
        self.payload_length.into()
    }

    /// Returns whether there are any payloads in this packet.
    pub fn is_empty(&self) -> bool {
        self.payload_length == 0
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
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Payload> {
        self.payloads[0..self.payload_length as usize].iter_mut()
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
        if self.payload_length >= 31 {
            return None;
        };
        self.payloads[self.payload_length as usize] = payload;
        self.payload_length += 1;
        Some(())
    }

    /// Try to append all the given [Payload]s to a [Packet]
    ///
    /// Fails if the final packet size would exceed 255 bytes (31 payloads).
    /// On failure, the packet was left unmodified.
    pub fn try_append_from_slice(&mut self, payloads: &[Payload]) -> Option<()> {
        if (self.payload_length as usize + payloads.len()) * 8 + 4 >= u8::MAX as usize {
            return None;
        };
        self.payloads[self.payload_length as usize..self.payload_length as usize + payloads.len()]
            .clone_from_slice(payloads);
        self.payload_length += payloads.len() as u8;
        Some(())
    }

    /// Serialize this Packet into a `&[u8]` which can be sent on-the-wire.
    ///
    /// This can fail if buf is to small, in which case `None` is returned.
    /// Otherwise, return the amount of bytes written into `buf`.
    pub fn try_serialize_into(&self, buf: &mut [u8]) -> Option<usize> {
        if buf.len() < 4 + self.payload_length as usize * 8 {
            return None;
        };
        // the HEADER
        buf[0] = self.version.major;
        buf[1] = self.version.minor;
        buf[2] = 4 + self.payload_length * 8;
        buf[3] = self.payload_length;

        // the PAYLOAD
        // now set each individual payload
        for (index, payload) in self
            .payloads
            .iter()
            .enumerate()
            .take_while(|(i, _)| *i < self.payload_length as usize)
        {
            payload.serialize_into(&mut buf[4 + index * 8..=11 + index * 8]);
        }
        Some(4 + self.payloads.len() * 8)
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
        let mut payloads = [crate::Payload::default(); 31];
        payloads[0] = crate::Payload {
            node: 3,
            pdo_index: 0,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeCentigrade_Tens(95)),
        };
        payloads[1] = crate::Payload {
            node: 3,
            pdo_index: 0,
            value: crate::COEValue::Digital(crate::DigitalCOEValue::OnOff(true)),
        };
        assert_eq!(
            packet,
            crate::Packet {
                version: crate::COEVersion { major: 2, minor: 0 },
                payloads,
                payload_length: 2
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
