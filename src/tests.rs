#![cfg(test)]

use crate::Packet;

use super::errors;

#[test]
fn parse_value_00() {
    let raw_bytes = [1, 1, 1, 0, 3, 2, 1, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 1, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Dimensionless(66051))});
}

#[test]
fn parse_value_01() {
    let raw_bytes = [1, 1, 1, 1, 154, 255, 255, 255];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 1, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeCentigrade_Tens(-102))});
}

#[test]
fn parse_value_02() {
    let raw_bytes = [1, 1, 1, 2, 123, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 1, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::WattPerSquareMeter(123))});
}

#[test]
fn parse_value_03() {
    let raw_bytes = [1, 17, 1, 3, 123, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 17, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerHour(123))});
}

#[test]
fn parse_value_04() {
    let raw_bytes = [7, 8, 1, 4, 57, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 7, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Seconds(57))});
}

#[test]
fn parse_value_05() {
    let raw_bytes = [7, 8, 1, 5, 12, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 7, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Minutes(12))});
}

#[test]
fn parse_value_06() {
    let raw_bytes = [7, 58, 1, 6, 3, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 7, pdo_index: 58, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerPulse_Tens(3))});
}

#[test]
fn parse_value_07() {
    let raw_bytes = [7, 58, 1, 7, 7, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 7, pdo_index: 58, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeKelvin_Tens(263))});
}

#[test]
fn parse_value_08() {
    let raw_bytes = [7, 58, 1, 8, 0, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 7, pdo_index: 58, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Percent_Tens(256))});
}

#[test]
fn parse_value_09() {
    let raw_bytes = [16, 8, 1, 9, 0, 1, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256))});
}

#[test]
fn parse_value_10() {
    let raw_bytes = [16, 8, 1, 10, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloWatt_Hundreds(4 * 256))});
}

#[test]
fn parse_value_11() {
    let raw_bytes = [16, 8, 1, 11, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KilowattHour_Tens(4 * 256))});
}

#[test]
fn parse_value_12() {
    let raw_bytes = [16, 8, 1, 12, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MegawattHour(4 * 256))});
}

#[test]
fn parse_value_13() {
    let raw_bytes = [16, 8, 1, 13, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Volt_Hundreds(4 * 256))});
}

#[test]
fn parse_value_14() {
    let raw_bytes = [16, 8, 1, 14, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliAmpere_Tens(4 * 256))});
}

#[test]
fn parse_value_15() {
    let raw_bytes = [16, 8, 1, 15, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Hours(4 * 256))});
}

#[test]
fn parse_value_16() {
    let raw_bytes = [16, 8, 1, 16, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Days(4 * 256))});
}

#[test]
fn parse_value_17() {
    let raw_bytes = [16, 8, 1, 17, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Pulses(4 * 256))});
}

#[test]
fn parse_value_18() {
    let raw_bytes = [16, 8, 1, 18, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloOhm_Hundreds(4 * 256))});
}

#[test]
fn parse_value_19() {
    let raw_bytes = [16, 8, 1, 19, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Liters(4 * 256))});
}

#[test]
fn parse_value_20() {
    let raw_bytes = [16, 8, 1, 20, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloMetersPerHour(4 * 256))});
}

#[test]
fn parse_value_21() {
    let raw_bytes = [16, 8, 1, 21, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Hertz_Hundreds(4 * 256))});
}

#[test]
fn parse_value_22() {
    let raw_bytes = [16, 8, 1, 22, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerMinute(4 * 256))});
}

#[test]
fn parse_value_23() {
    let raw_bytes = [16, 8, 1, 23, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Bar_Hundreds(4 * 256))});
}

#[test]
fn parse_value_24() {
    let raw_bytes = [16, 8, 1, 24, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CoefficientOfPerformance_Hundreds(4 * 256))});
}

#[test]
fn parse_value_25() {
    let raw_bytes = [16, 8, 1, 25, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloMeter(4 * 256))});
}

#[test]
fn parse_value_26() {
    let raw_bytes = [16, 8, 1, 26, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Meter(4 * 256))});
}

#[test]
fn parse_value_27() {
    let raw_bytes = [16, 8, 1, 27, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeter(4 * 256))});
}

#[test]
fn parse_value_28() {
    let raw_bytes = [16, 8, 1, 28, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeter(4 * 256))});
}

#[test]
fn parse_value_29() {
    let raw_bytes = [16, 8, 1, 29, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HertzPerKiloMeterPerHour_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_30() {
    let raw_bytes = [16, 8, 1, 30, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HertzPerMeterPerSecond_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_31() {
    let raw_bytes = [16, 8, 1, 31, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KilowattHourPerPulse_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_32() {
    let raw_bytes = [16, 8, 1, 32, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerPulse_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_33() {
    let raw_bytes = [16, 8, 1, 33, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerPulse_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_34() {
    let raw_bytes = [16, 8, 1, 34, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerPulse_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_35() {
    let raw_bytes = [16, 8, 1, 35, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::LiterPerDay(4 * 256))});
}

#[test]
fn parse_value_36() {
    let raw_bytes = [16, 8, 1, 36, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MetersPerSecond(4 * 256))});
}

#[test]
fn parse_value_37() {
    let raw_bytes = [16, 8, 1, 37, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerMinute(4 * 256))});
}

#[test]
fn parse_value_38() {
    let raw_bytes = [16, 8, 1, 38, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerHour(4 * 256))});
}

#[test]
fn parse_value_39() {
    let raw_bytes = [16, 8, 1, 39, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CubicMeterPerDay(4 * 256))});
}

#[test]
fn parse_value_40() {
    let raw_bytes = [16, 8, 1, 40, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerMinute(4 * 256))});
}

#[test]
fn parse_value_41() {
    let raw_bytes = [16, 8, 1, 41, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerHour(4 * 256))});
}

#[test]
fn parse_value_42() {
    let raw_bytes = [16, 8, 1, 42, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MilliMeterPerDay(4 * 256))});
}

#[test]
fn parse_value_43() {
    let raw_bytes = [16, 8, 0, 43, 0, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Digital(crate::DigitalCOEValue::DigitalOnOff(false))});
}

#[test]
fn parse_value_44() {
    let raw_bytes = [1, 63, 0, 44, 1, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 63, value: crate::COEValue::Digital(crate::DigitalCOEValue::DigitalNoYes(true))});
}

#[test]
fn parse_value_45() {
    let raw_bytes = [16, 8, 1, 45, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::RASMode(4 * 256))});
}

#[test]
fn parse_value_46() {
    let raw_bytes = [16, 8, 1, 46, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeCentigradePlusRAS_Tens(4 * 256))});
}

#[test]
fn parse_value_47() {
    let raw_bytes = [16, 8, 1, 47, 1, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Mixer(1))});
}

#[test]
fn parse_value_48() {
    let raw_bytes = [16, 8, 1, 48, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HeatingCircuitOpMode(4 * 256))});
}

#[test]
fn parse_value_49() {
    let raw_bytes = [16, 8, 1, 49, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::HeatingCircuitOpLevel(4 * 256))});
}

#[test]
fn parse_value_50() {
    let raw_bytes = [16, 8, 1, 50, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CurrencyEuro_Hundreds(4 * 256))});
}

#[test]
fn parse_value_51() {
    let raw_bytes = [16, 8, 1, 51, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CurrencyDollar_Hundreds(4 * 256))});
}

#[test]
fn parse_value_52() {
    let raw_bytes = [16, 8, 1, 52, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::AbsoluteHumidity_Tens(4 * 256))});
}

#[test]
fn parse_value_53() {
    let raw_bytes = [16, 8, 1, 53, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::PricePerUnit_HundredThousands(4 * 256))});
}

#[test]
fn parse_value_54() {
    let raw_bytes = [16, 8, 1, 54, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Degree_Tens(4 * 256))});
}

#[test]
fn parse_value_55() {
    let raw_bytes = [16, 8, 1, 55, 0, 0, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Blinds(0))});
}

#[test]
fn parse_value_56() {
    let raw_bytes = [16, 8, 1, 56, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Degree_Millions(4 * 256))});
}

#[test]
fn parse_value_57() {
    let raw_bytes = [16, 8, 1, 57, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Second_Tens(4 * 256))});
}

#[test]
fn parse_value_58() {
    let raw_bytes = [16, 8, 1, 58, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Dimensionless_Tens(4 * 256))});
}

#[test]
fn parse_value_59() {
    let raw_bytes = [16, 8, 1, 59, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::BlindsPosition(4 * 256))});
}

#[test]
fn parse_value_60() {
    let raw_bytes = [16, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))});
}

#[test]
fn parse_value_61() {
    let raw_bytes = [16, 8, 1, 61, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DayOfMonth(4 * 256))});
}

#[test]
fn parse_value_62() {
    let raw_bytes = [16, 8, 1, 62, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Date(4 * 256))});
}

#[test]
fn parse_value_63() {
    let raw_bytes = [16, 8, 1, 63, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Ampere_Tens(4 * 256))});
}

#[test]
fn parse_value_64() {
    let raw_bytes = [16, 8, 1, 64, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::MonthOfYear(4 * 256))});
}

#[test]
fn parse_value_65() {
    let raw_bytes = [16, 8, 1, 65, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Millibar_Tens(4 * 256))});
}

#[test]
fn parse_value_66() {
    let raw_bytes = [16, 8, 1, 66, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Pascal(4 * 256))});
}

#[test]
fn parse_value_67() {
    let raw_bytes = [16, 8, 1, 67, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CO2Content(4 * 256))});
}

#[test]
fn parse_value_68() {
    let raw_bytes = [16, 8, 1, 68, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::RawHex(4 * 256))});
}

#[test]
fn parse_value_69() {
    let raw_bytes = [16, 8, 1, 69, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Watt(4 * 256))});
}

#[test]
fn parse_value_70() {
    let raw_bytes = [16, 8, 1, 70, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Tonne_Hundreds(4 * 256))});
}

#[test]
fn parse_value_71() {
    let raw_bytes = [16, 8, 1, 71, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::KiloGram_Tens(4 * 256))});
}

#[test]
fn parse_value_72() {
    let raw_bytes = [16, 8, 1, 72, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Gram_Tens(4 * 256))});
}

#[test]
fn parse_value_73() {
    let raw_bytes = [16, 8, 1, 73, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::CentiMeter_Tens(4 * 256))});
}

#[test]
fn parse_value_74() {
    let raw_bytes = [16, 8, 1, 74, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::ColourTemperature(4 * 256))});
}

#[test]
fn parse_value_75() {
    let raw_bytes = [16, 8, 1, 75, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Lux_Tens(4 * 256))});
}


#[test]
fn parse_payload_frame_to_small() {
    let raw_bytes = [16, 8, 1, 0, 4, 0, 0];
    let payload: Result<crate::Payload, _> = raw_bytes[0..7].try_into();
    assert_eq!(payload, Err(errors::ParseCOEError::PayloadFrameLengthIncorrect(7)));
}

#[test]
fn parse_payload_frame_to_large() {
    let raw_bytes = [16, 8, 1, 0, 0, 4, 3, 2, 1, 0];
    let payload: Result<crate::Payload, _> = raw_bytes[0..10].try_into();
    assert_eq!(payload, Err(errors::ParseCOEError::PayloadFrameLengthIncorrect(10)));
}

#[test]
fn parse_value_incorrect_length() {
    let unit_id = 0;
    let raw_bytes = [1, 0, 0, 4, 0];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..5]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::ValueSize(5_usize)));
}

#[test]
fn parse_value_not_bool() {
    let unit_id = 43;
    let raw_bytes = [0, 0, 1, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::ValueNotBool(raw_bytes)));
}

#[test]
fn parse_value_digital_but_unit_is_analogue() {
    let unit_id = 0;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::FormatAndUnitIncompatible("The Unit ID 0 is not known as a digital Unit.".to_string())));
}

#[test]
fn parse_value_analogue_but_unit_is_digital() {
    let unit_id = 43;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::FormatAndUnitIncompatible("The Unit ID 43 is not known as an analogue Unit.".to_string())));
}

#[test]
fn parse_value_digital_unit_does_not_exist() {
    let unit_id = 123;
    let raw_bytes = [1, 0, 0, 0];
    let value: Result<crate::DigitalCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::FormatAndUnitIncompatible("The Unit ID 123 is not known as a digital Unit.".to_string())));
}

#[test]
fn parse_value_analogue_unit_does_not_exist() {
    let unit_id = 123;
    let raw_bytes = [0, 0, 0, 1];
    let value: Result<crate::AnalogueCOEValue, _> = (&unit_id, &raw_bytes[0..4]).try_into();
    assert_eq!(value, Err(errors::ParseCOEError::FormatAndUnitIncompatible("The Unit ID 123 is not known as an analogue Unit.".to_string())));
}

#[test]
fn parse_payload_node_to_small() {
    let raw_bytes = [0_u8, 8, 1, 75, 0, 0, 4, 0];
    let error: errors::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8]).expect_err("This Packet is not parsable.");
    assert_eq!(
        error,
        errors::ParseCOEError::NodeDisallowed(0u8));
}

#[test]
fn parse_payload_node_to_large() {
    let raw_bytes = [63_u8, 8, 1, 75, 0, 0, 4, 0];
    let error: errors::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8]).expect_err("This Packet is not parsable.");
    assert_eq!(
        error,
        errors::ParseCOEError::NodeDisallowed(63_u8));
}


#[test]
fn parse_payload_node_1() {
    let raw_bytes = [1, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))});
}

#[test]
fn parse_payload_node_62() {
    let raw_bytes = [62, 8, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 62, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))});
}

#[test]
fn parse_payload_pdo_to_large() {
    let raw_bytes = [12_u8, 64, 1, 75, 0, 4, 0, 0];
    let error: errors::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8]).expect_err("This Packet is not parsable.");
    assert_eq!(
        error,
        errors::ParseCOEError::PDOIndexDisallowed(64_u8));
}

#[test]
fn parse_payload_pdo_0() {
    let raw_bytes = [1, 0, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 1, pdo_index: 0, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))});
}

#[test]
fn parse_payload_pdo_63() {
    let raw_bytes = [62, 63, 1, 60, 0, 4, 0, 0];
    let payload: crate::Payload = raw_bytes[0..8].try_into().expect("This Packet is parsable.");
    assert_eq!(
        payload,
        crate::Payload{node: 62, pdo_index: 63, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Time(4 * 256))});
}

#[test]
fn parse_payload_format_unknown() {
    let raw_bytes = [12_u8, 4, 3, 75, 0, 4, 0, 0];
    let error: errors::ParseCOEError = TryInto::<crate::Payload>::try_into(&raw_bytes[0..8]).expect_err("This Packet is not parsable.");
    assert_eq!(
        error,
        errors::ParseCOEError::FormatUnknown(3_u8));
}

#[test]
fn parse_packet_success() {
    let raw_bytes = [2, 0, 20, 2, 3, 0, 1, 1, 95, 0, 0, 0, 3, 0, 0, 43, 1, 0, 0, 0];
    let packet: crate::Packet = raw_bytes[0..20].try_into().expect("This Packet is parsable.");
    assert_eq!(
        packet,
        crate::Packet{
            version: crate::COEVersion{ major: 2, minor: 0}, 
            payload: vec![crate::Payload{node: 3, pdo_index: 0, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::DegreeCentigrade_Tens(95))},
                          crate::Payload{node: 3, pdo_index: 0, value: crate::COEValue::Digital(crate::DigitalCOEValue::DigitalOnOff(true))}]});
}

#[test]
fn parse_packet_below_header_length() {
    let raw_bytes = [2, 0, 20];
    let err: errors::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..3]).expect_err("This Packet is not parsable.");
    assert_eq!(
        err, errors::ParseCOEError::PacketBelowHeaderLength);
}

#[test]
fn parse_packet_packet_length_inconsistent() {
    let raw_bytes = [2, 0, 21, 2, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1];
    let err: errors::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20]).expect_err("This Packet is not parsable");
    assert_eq!(
        err, errors::ParseCOEError::PacketLengthInconsistent(21_u8, 2_u8));
}

#[test]
fn parse_packet_packet_length_inconsistent_2() {
    let raw_bytes = [2, 0, 20, 3, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1];
    let err: errors::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20]).expect_err("This Packet is not parsable");
    assert_eq!(
        err, errors::ParseCOEError::PacketLengthInconsistent(20_u8, 3_u8));
}

#[test]
fn parse_packet_packet_size_conflicts_with_header() {
    let raw_bytes = [2, 0, 12, 1, 3, 0, 1, 1, 0, 0, 0, 95, 3, 0, 0, 43, 0, 0, 0, 1];
    let err: errors::ParseCOEError = TryInto::<crate::Packet>::try_into(&raw_bytes[0..20]).expect_err("This Packet is not parsable");
    assert_eq!(
        err, errors::ParseCOEError::PacketSizeConflictsWithHeader(12, 20));
}

#[test]
fn parse_version_implemented() {
    let major = 2;
    let minor = 0;
    let version: crate::COEVersion = (major, minor).try_into().expect("Version 2.0 is implemented");
    assert_eq!(version, crate::COEVersion{ major, minor });
}

#[test]
fn parse_version_not_implemented() {
    let major = 0;
    let minor = 1;
    let version: errors::ParseCOEError = TryInto::<crate::COEVersion>::try_into((major, minor)).expect_err("Version 0.1 is not implemented");
    assert_eq!(version, errors::ParseCOEError::VersionNotImplemented(major, minor));
}

#[test]
fn deser_ser() {
    let raw_bytes = [2, 0, 20, 2, 3, 0, 1, 1, 95, 0, 0, 0, 3, 0, 0, 43, 1, 0, 0, 0];
    let packet: crate::Packet = raw_bytes[0..20].try_into().expect("This Packet is parsable.");
    let re_serialized: Vec<u8> = packet.into();
    assert_eq!(re_serialized, raw_bytes);
}

#[test]
fn too_many_payloads() {
        let payload = crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256))};
        let payloads = [payload; 64];
        let res = Packet::try_from_payloads(&payloads);
        assert_eq!(res, Err(crate::errors::PacketMaxPayloadsExceeded{}));
}

#[test]
fn packets_from_payloads() {
        let payload = crate::Payload{node: 16, pdo_index: 8, value: crate::COEValue::Analogue(crate::AnalogueCOEValue::Colon(256))};
        let payloads = [payload; 64];
        let packets = super::packets_from_payloads(&payloads);
        assert_eq!(packets.len(), 3);
}
