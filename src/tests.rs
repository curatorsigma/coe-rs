#![cfg(test)]

use crate::*;

#[test]
fn parse_value_00() {
    let raw_bytes = [1, 1, 1, 0, 3, 2, 1, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 1,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Dimensionless(66051))
        }
    );
}

#[test]
fn parse_value_01() {
    let raw_bytes = [1, 1, 1, 1, 154, 255, 255, 255];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 1);
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 1,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeCentigrade_Tens(-102))
        }
    );
}

#[test]
fn parse_value_02() {
    let raw_bytes = [1, 1, 1, 2, 123, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 2);
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 1,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::WattPerSquareMeter(123))
        }
    );
}

#[test]
fn parse_value_03() {
    let raw_bytes = [1, 17, 1, 3, 123, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 3);
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 17,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerHour(123))
        }
    );
}

#[test]
fn parse_value_04() {
    let raw_bytes = [7, 8, 1, 4, 57, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 4);
    assert_eq!(
        payload,
        crate::Payload {
            node: 7,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Seconds(57))
        }
    );
}

#[test]
fn parse_value_05() {
    let raw_bytes = [7, 8, 1, 5, 12, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 5);
    assert_eq!(
        payload,
        crate::Payload {
            node: 7,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Minutes(12))
        }
    );
}

#[test]
fn parse_value_06() {
    let raw_bytes = [7, 58, 1, 6, 3, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 6);
    assert_eq!(
        payload,
        crate::Payload {
            node: 7,
            pdo_index: 58,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerPulse_Tens(3))
        }
    );
}

#[test]
fn parse_value_07() {
    let raw_bytes = [7, 58, 1, 7, 7, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 7);
    assert_eq!(
        payload,
        crate::Payload {
            node: 7,
            pdo_index: 58,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeKelvin_Tens(263))
        }
    );
}

#[test]
fn parse_value_08() {
    let raw_bytes = [7, 58, 1, 8, 0, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 8);
    assert_eq!(
        payload,
        crate::Payload {
            node: 7,
            pdo_index: 58,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Percent_Tens(256))
        }
    );
}

#[test]
fn parse_value_09() {
    let raw_bytes = [16, 8, 1, 9, 0, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 9);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256))
        }
    );
}

#[test]
fn parse_value_10() {
    let raw_bytes = [16, 8, 1, 10, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 10);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloWatt_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_11() {
    let raw_bytes = [16, 8, 1, 11, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 11);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KilowattHour_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_12() {
    let raw_bytes = [16, 8, 1, 12, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 12);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MegawattHour(4 * 256))
        }
    );
}

#[test]
fn parse_value_13() {
    let raw_bytes = [16, 8, 1, 13, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 13);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Volt_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_14() {
    let raw_bytes = [16, 8, 1, 14, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 14);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliAmpere_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_15() {
    let raw_bytes = [16, 8, 1, 15, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 15);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Hours(4 * 256))
        }
    );
}

#[test]
fn parse_value_16() {
    let raw_bytes = [16, 8, 1, 16, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 16);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Days(4 * 256))
        }
    );
}

#[test]
fn parse_value_17() {
    let raw_bytes = [16, 8, 1, 17, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 17);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Pulses(4 * 256))
        }
    );
}

#[test]
fn parse_value_18() {
    let raw_bytes = [16, 8, 1, 18, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 18);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloOhm_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_19() {
    let raw_bytes = [16, 8, 1, 19, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 19);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Liters(4 * 256))
        }
    );
}

#[test]
fn parse_value_20() {
    let raw_bytes = [16, 8, 1, 20, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 20);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloMetersPerHour(4 * 256))
        }
    );
}

#[test]
fn parse_value_21() {
    let raw_bytes = [16, 8, 1, 21, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 21);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Hertz_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_22() {
    let raw_bytes = [16, 8, 1, 22, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 22);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerMinute(4 * 256))
        }
    );
}

#[test]
fn parse_value_23() {
    let raw_bytes = [16, 8, 1, 23, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 23);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Bar_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_24() {
    let raw_bytes = [16, 8, 1, 24, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 24);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::CoefficientOfPerformance_Hundreds(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_25() {
    let raw_bytes = [16, 8, 1, 25, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 25);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloMeter(4 * 256))
        }
    );
}

#[test]
fn parse_value_26() {
    let raw_bytes = [16, 8, 1, 26, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 26);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Meter_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_27() {
    let raw_bytes = [16, 8, 1, 27, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 27);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeter(4 * 256))
        }
    );
}

#[test]
fn parse_value_28() {
    let raw_bytes = [16, 8, 1, 28, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 28);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeter(4 * 256))
        }
    );
}

#[test]
fn parse_value_29() {
    let raw_bytes = [16, 8, 1, 29, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 29);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::HertzPerKiloMeterPerHour_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_30() {
    let raw_bytes = [16, 8, 1, 30, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 30);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::HertzPerMeterPerSecond_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_31() {
    let raw_bytes = [16, 8, 1, 31, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 31);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::KilowattHourPerPulse_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_32() {
    let raw_bytes = [16, 8, 1, 32, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 32);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::CubicMeterPerPulse_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_33() {
    let raw_bytes = [16, 8, 1, 33, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 33);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::MilliMeterPerPulse_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_34() {
    let raw_bytes = [16, 8, 1, 34, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 34);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::LiterPerPulse_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_35() {
    let raw_bytes = [16, 8, 1, 35, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 35);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerDay(4 * 256))
        }
    );
}

#[test]
fn parse_value_36() {
    let raw_bytes = [16, 8, 1, 36, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 36);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MetersPerSecond(4 * 256))
        }
    );
}

#[test]
fn parse_value_37() {
    let raw_bytes = [16, 8, 1, 37, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 37);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerMinute(4 * 256))
        }
    );
}

#[test]
fn parse_value_38() {
    let raw_bytes = [16, 8, 1, 38, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 38);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerHour(4 * 256))
        }
    );
}

#[test]
fn parse_value_39() {
    let raw_bytes = [16, 8, 1, 39, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 39);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerDay(4 * 256))
        }
    );
}

#[test]
fn parse_value_40() {
    let raw_bytes = [16, 8, 1, 40, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 40);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerMinute_Tens(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_41() {
    let raw_bytes = [16, 8, 1, 41, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 41);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerHour_Tens(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_42() {
    let raw_bytes = [16, 8, 1, 42, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 42);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerDay_Tens(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_43() {
    let raw_bytes = [16, 8, 0, 43, 0, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 43);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Digital(crate::DigitalCOEValue::OnOff(false))
        }
    );
}

#[test]
fn parse_value_44() {
    let raw_bytes = [1, 63, 0, 44, 1, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 44);
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 63,
            value: crate::COEValue::Digital(crate::DigitalCOEValue::YesNo(true))
        }
    );
}

#[test]
fn parse_value_45() {
    let raw_bytes = [16, 8, 0, 45, 1, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 45);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Digital(crate::DigitalCOEValue::RASMode(true))
        }
    );
}

#[test]
fn parse_value_46() {
    let raw_bytes = [16, 8, 1, 46, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 46);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::DegreeCentigradePlusRAS_Tens(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_47() {
    let raw_bytes = [16, 8, 0, 47, 0, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 47);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Digital(crate::DigitalCOEValue::Mixer(false))
        }
    );
}

#[test]
fn parse_value_48() {
    let raw_bytes = [16, 8, 1, 48, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 48);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HeatingCircuitOpMode(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_49() {
    let raw_bytes = [16, 8, 1, 49, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 49);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HeatingCircuitOpLevel(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_50() {
    let raw_bytes = [16, 8, 1, 50, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 50);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CurrencyEuro_Hundreds(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_51() {
    let raw_bytes = [16, 8, 1, 51, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 51);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CurrencyDollar_Hundreds(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_52() {
    let raw_bytes = [16, 8, 1, 52, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 52);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::AbsoluteHumidity_Tens(
                4 * 256
            ))
        }
    );
}

#[test]
fn parse_value_53() {
    let raw_bytes = [16, 8, 1, 53, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 53);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(
                crate::AnalogueCOEValue::PricePerUnit_HundredThousands(4 * 256)
            )
        }
    );
}

#[test]
fn parse_value_54() {
    let raw_bytes = [16, 8, 1, 54, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 54);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Degree_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_55() {
    let raw_bytes = [16, 8, 1, 55, 0, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 55);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Blinds(0))
        }
    );
}

#[test]
fn parse_value_56() {
    let raw_bytes = [16, 8, 1, 56, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 56);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Degree_Millions(4 * 256))
        }
    );
}

#[test]
fn parse_value_57() {
    let raw_bytes = [16, 8, 1, 57, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 57);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Second_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_58() {
    let raw_bytes = [16, 8, 1, 58, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 58);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Dimensionless_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_59() {
    let raw_bytes = [16, 8, 1, 59, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 59);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::BlindsPosition(4 * 256))
        }
    );
}

#[test]
fn parse_value_60() {
    let raw_bytes = [16, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 60);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))
        }
    );
}

#[test]
fn parse_value_61() {
    let raw_bytes = [16, 8, 1, 61, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 61);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DayOfMonth(4 * 256))
        }
    );
}

#[test]
fn parse_value_62() {
    let raw_bytes = [16, 8, 1, 62, 29, 5, 173, 5];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 62);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Date(29, 5, 1453))
        }
    );
}

#[test]
fn parse_value_63() {
    let raw_bytes = [16, 8, 1, 63, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 63);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Ampere_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_64() {
    let raw_bytes = [16, 8, 1, 64, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 64);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MonthOfYear(4 * 256))
        }
    );
}

#[test]
fn parse_value_65() {
    let raw_bytes = [16, 8, 1, 65, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 65);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Millibar_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_66() {
    let raw_bytes = [16, 8, 1, 66, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 66);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Pascal(4 * 256))
        }
    );
}

#[test]
fn parse_value_67() {
    let raw_bytes = [16, 8, 1, 67, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 67);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CO2Content(4 * 256))
        }
    );
}

#[test]
fn parse_value_68() {
    let raw_bytes = [16, 8, 1, 68, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 68);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::RawHex(4 * 256))
        }
    );
}

#[test]
fn parse_value_69() {
    let raw_bytes = [16, 8, 1, 69, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 69);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Watt(4 * 256))
        }
    );
}

#[test]
fn parse_value_70() {
    let raw_bytes = [16, 8, 1, 70, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 70);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Tonne_Hundreds(4 * 256))
        }
    );
}

#[test]
fn parse_value_71() {
    let raw_bytes = [16, 8, 1, 71, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 71);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloGram_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_72() {
    let raw_bytes = [16, 8, 1, 72, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 72);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Gram_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_73() {
    let raw_bytes = [16, 8, 1, 73, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 73);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CentiMeter_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_value_74() {
    let raw_bytes = [16, 8, 1, 74, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 74);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::ColourTemperature(4 * 256))
        }
    );
}

#[test]
fn parse_value_75() {
    let raw_bytes = [16, 8, 1, 75, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 75);
    assert_eq!(
        payload,
        crate::Payload {
            node: 16,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Lux_Tens(4 * 256))
        }
    );
}

#[test]
fn parse_payload_frame_to_small() {
    let raw_bytes = [16, 8, 1, 0, 4, 0, 0];
    let payload: Result<crate::Payload, _> = raw_bytes[0..7].try_into();
    assert_eq!(
        payload,
        Err(crate::ParseCOEError::PayloadFrameLengthIncorrect(7))
    );
}

#[test]
fn parse_payload_frame_to_large() {
    let raw_bytes = [16, 8, 1, 0, 0, 4, 3, 2, 1, 0];
    let payload: Result<crate::Payload, _> = raw_bytes[0..10].try_into();
    assert_eq!(
        payload,
        Err(crate::ParseCOEError::PayloadFrameLengthIncorrect(10))
    );
}

#[test]
fn parse_value_incorrect_length() {
    let unit_id = 0;
    let raw_bytes = [1, 0, 0, 4, 0];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..5]).try_into();
    assert_eq!(value, Err(crate::ParseCOEError::ValueSize(5_usize)));
}

#[test]
fn parse_value_not_bool() {
    let unit_id = 43;
    let raw_bytes = [0, 0, 1, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(crate::ParseCOEError::ValueNotBool(raw_bytes)));
}

#[test]
fn parse_value_digital_but_unit_is_analogue() {
    let unit_id = 0;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(
        value,
        Err(crate::ParseCOEError::FormatAndUnitIncompatible(
            Format::Digital,
            0
        ))
    );
}

#[test]
fn parse_value_analogue_but_unit_is_digital() {
    let unit_id = 43;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(
        value,
        Err(crate::ParseCOEError::FormatAndUnitIncompatible(
            Format::Analogue,
            43
        ))
    );
}

#[test]
fn parse_value_digital_unit_does_not_exist() {
    let unit_id = 123;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(
        value,
        Err(crate::ParseCOEError::FormatAndUnitIncompatible(
            Format::Digital,
            123
        ))
    );
}

#[test]
fn parse_value_analogue_unit_does_not_exist() {
    let unit_id = 123;
    let raw_bytes = [0, 0, 0, 1];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(
        value,
        Err(crate::ParseCOEError::FormatAndUnitIncompatible(
            Format::Analogue,
            123
        ))
    );
}

#[test]
fn parse_payload_node_to_small() {
    let raw_bytes = [0_u8, 8, 1, 75, 0, 0, 4, 0];
    let error: crate::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8])
        .expect_err("This Packet is not parsable.");
    assert_eq!(error, crate::ParseCOEError::NodeDisallowed(0u8));
}

#[test]
fn parse_payload_node_to_large() {
    let raw_bytes = [63_u8, 8, 1, 75, 0, 0, 4, 0];
    let error: crate::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8])
        .expect_err("This Packet is not parsable.");
    assert_eq!(error, crate::ParseCOEError::NodeDisallowed(63_u8));
}

#[test]
fn parse_payload_node_1() {
    let raw_bytes = [1, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))
        }
    );
}

#[test]
fn parse_payload_node_62() {
    let raw_bytes = [62, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload {
            node: 62,
            pdo_index: 8,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))
        }
    );
}

#[test]
fn parse_payload_pdo_to_large() {
    let raw_bytes = [12_u8, 64, 1, 75, 0, 4, 0, 0];
    let error: crate::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8])
        .expect_err("This Packet is not parsable.");
    assert_eq!(error, crate::ParseCOEError::PDOIndexDisallowed(64_u8));
}

#[test]
fn parse_payload_pdo_0() {
    let raw_bytes = [1, 0, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 60);
    assert_eq!(
        payload,
        crate::Payload {
            node: 1,
            pdo_index: 0,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))
        }
    );
}

#[test]
fn parse_payload_pdo_63() {
    let raw_bytes = [62, 63, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(payload.unit_id(), 60);
    assert_eq!(
        payload,
        crate::Payload {
            node: 62,
            pdo_index: 63,
            value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))
        }
    );
}

#[test]
fn parse_payload_format_unknown() {
    let raw_bytes = [12_u8, 4, 3, 75, 0, 4, 0, 0];
    let error: crate::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8])
        .expect_err("This Packet is not parsable.");
    assert_eq!(error, crate::ParseCOEError::FormatUnknown(3_u8));
}

#[test]
fn parse_version_implemented() {
    let major = 2;
    let minor = 0;
    let version: crate::COEVersion = (major, minor)
        .try_into()
        .expect("Version 2.0 is implemented");
    assert_eq!(version, crate::COEVersion { major, minor });
}

#[test]
fn parse_version_not_implemented() {
    let major = 0;
    let minor = 1;
    let version: crate::ParseCOEError = TryInto::<crate::COEVersion>::try_into((major, minor))
        .expect_err("Version 0.1 is not implemented");
    assert_eq!(
        version,
        crate::ParseCOEError::VersionNotImplemented(major, minor)
    );
}

#[test]
fn deser_ser() {
    let raw_bytes = [
        2, 0, 20, 2, 3, 0, 1, 1, 95, 0, 0, 0, 3, 0, 0, 43, 1, 0, 0, 0,
    ];
    let packet: crate::Packet = raw_bytes[0..20]
        .try_into()
        .expect("This Packet is parsable.");
    assert_eq!(packet.len(), 2);
    let mut re_serialized = [0_u8; 20];
    packet.try_serialize_into(&mut re_serialized).unwrap();
    assert_eq!(re_serialized, raw_bytes);
}

#[test]
fn too_many_payloads() {
    let payload = crate::Payload {
        node: 16,
        pdo_index: 8,
        value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256)),
    };
    let payloads = [payload; 32];
    let res = Packet::try_from_payloads(&payloads);
    assert_eq!(res, None);

    let mut res = Packet::try_from_payloads(&payloads[..31]).unwrap();
    assert_eq!(res.try_push(payload), None);
}

#[test]
#[cfg(feature = "alloc")]
fn packets_from_payloads() {
    let payload = crate::Payload {
        node: 16,
        pdo_index: 8,
        value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256)),
    };
    let payloads = [payload; 64];
    let packets = crate::packets_from_payloads(&payloads);
    assert_eq!(packets.len(), 3);
}

#[test]
fn packet_iteration() {
    let raw_bytes = [
        2, 0, 20, 2, 3, 0, 1, 1, 95, 0, 0, 0, 3, 0, 0, 43, 1, 0, 0, 0,
    ];
    let mut packet: crate::Packet = raw_bytes[0..20]
        .try_into()
        .expect("This Packet is parsable.");
    let version = packet.version();
    assert_eq!(version, COEVersion{ major: 2, minor: 0});

    let packet_iter = packet.iter();
    assert_eq!(packet_iter.count(), 2);

    let packet_iter = packet.iter_mut();
    assert_eq!(packet_iter.count(), 2);


    for _payload in packet {
    };
}
