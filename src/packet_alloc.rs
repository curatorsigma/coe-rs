//! Implement [Packet] with the alloc feature enabled

use super::*;

/// A COE Packet
///
/// Note: we enforce and assume that `payload.len()` never exceeds 31.
/// This is required, because the packet contains its own size (in bytes) in a field containing a
/// u8, so no more then 255 (`u8::MAX`) bytes may ever be contained in a packets full representation.
/// The packet on wire contains 4 bytes of headers, leaving us with 251 usable bytes. A payload
/// length of 8 byte per payload yields 31 full payloads that fit in the max packet length.
#[derive(Hash, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Packet {
    /// CoE Version used. Currently, only 2.0 is supported.
    version: COEVersion,
    /// The actual payloads.
    payload: Vec<Payload>,
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
        let mut payload: Vec<Payload> = vec![];
        for payload_nr in 0..value[3] {
            // each payload is exactly 8 bytes long - +4 is the header offset
            payload.push(
                value[(payload_nr * 8 + 4) as usize..=(payload_nr * 8 + 11) as usize].try_into()?,
            );
        }
        Ok(Packet { version, payload })
    }
}
impl From<Packet> for Vec<u8> {
    /// Serialize a packet into `Vec<u8>`
    ///
    /// This is guaranteed to succeed since a Packet can never have more then 31 payloads, such
    /// that the resulting serialization will always be at most 255 bytes long.
    fn from(value: Packet) -> Self {
        // precalculate the package size so we can allocate exactly the correct vector length
        let payload_length: u8 = value
            .payload
            .len()
            .try_into()
            .expect("Packet is larger then the largest possible COE frame allows.");
        let package_size: u8 = (payload_length as u16 * 8 + 4)
            .try_into()
            .expect("Packet is larger then the largest possible COE frame allows.");

        // we initialize this vector to all 0 and thus satisfy the requirements for
        // payload.serialize_into
        let mut res: Vec<u8> = vec![0; package_size as usize];

        // the HEADER
        res[0] = value.version.major;
        res[1] = value.version.minor;
        res[2] = package_size;
        res[3] = payload_length;

        // the PAYLOAD
        // now set each individual payload
        for (index, payload) in value.payload.iter().enumerate() {
            payload.serialize_into(&mut res[4 + index * 8..=11 + index * 8]);
        }
        return res;
    }
}
impl Packet {
    /// Create a packet without payloads
    pub fn new() -> Packet {
        Packet {
            version: COEVersion { major: 2, minor: 0 },
            payload: vec![],
        }
    }

    /// Create a [Packet] with [Payload]s. Fails if more then 31 payloads are given.
    pub fn try_from_payloads(payloads: &[Payload]) -> Result<Packet, PacketMaxPayloadsExceeded> {
        let mut p = Packet::new();
        p.try_append_from_slice(payloads)?;
        Ok(p)
    }

    /// Try to append a [Payload] to a [Packet]
    ///
    /// Fails if the final packet size would exceed 255 bytes (31 payloads).
    /// On failure, the packet was left unmodified.
    pub fn try_push(&mut self, payload: Payload) -> Result<(), PacketMaxPayloadsExceeded> {
        if self.payload.len() * 8 + 4 >= u8::MAX as usize {
            return Err(PacketMaxPayloadsExceeded {});
        };
        self.payload.push(payload);
        Ok(())
    }

    /// Try to append all the given [Payload]s to a [Packet]
    ///
    /// Fails if the final packet size would exceed 255 bytes (31 payloads).
    /// On failure, the packet was left unmodified.
    pub fn try_append_from_slice(
        &mut self,
        payloads: &[Payload],
    ) -> Result<(), PacketMaxPayloadsExceeded> {
        if (self.payload.len() + payloads.len()) * 8 + 4 >= u8::MAX as usize {
            return Err(PacketMaxPayloadsExceeded {});
        };
        self.payload.extend_from_slice(payloads);
        Ok(())
    }
}


mod test {
    use super::*;

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
                payload: vec![
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

