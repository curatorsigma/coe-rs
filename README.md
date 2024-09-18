![cargo test](https://github.com/curatorsigma/coe-rs/actions/workflows/rust.yml/badge.svg)

# coe-rs
`coe-rs` is an implementation of the full CAN-over-Ethernet spec by Technische Alternative, written in 100% safe rust with a `no_std` version available.
It allows safe (De-)serialization of COE packets from(into) bytes.

# Getting started
`coe-rs` is as small as possible and only handles (De-)serialization of CoE packets.
To use the protocol over a network, consider this minimal example:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut test_packet = Packet::new();
    // send to CAN-ID 58, at offset 1 (shows up as 2 in the GUI)
    test_packet.try_push(Payload::new(58, 1, coe::COEValue::Analogue(AnalogueCOEValue::LiterPerPulse_Tens(123))))?;

    let socket = UdpSocket::bind("0.0.0.0:34215").await?;
    let mut buf = [0_u8; 252];
    test_packet.try_serialize_into(&mut buf).expect("252 bytes is always large enough to fit a
    CoE Packet");
    // connect to the IP of your CMI
    socket.connect("192.168.1.123:5442").await?;
    socket.send(&buf).await?;
    Ok(())
}
```

You can receive packets like this:
```rust
async fn listener() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:5442").await?;

    // the largest possible COE packet is 256 byte long, so this is always safe
    let mut buf = [0_u8; 256];
    loop {
        let (length, _) = socket.recv_from(&mut buf).await?;
        let parsed = TryInto::<coe::Packet>::try_into(&buf[0..length])?;
        dbg!(&parsed);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_handle = tokio::spawn(async { listener().await });
    let (res,) = tokio::join!(listen_handle);
    Ok(())
}
```

You can find a real-world application of `coe-rs` in [churchtools-ta-sync](https://github.com/curatorsigma/churchtools-ta-sync).
Where we continually push data from an sqlite database to CMIs.

# The CoE protocol
## IMPORTANT LEGAL NOTE
The CoE Protocol is intellectual property of `Technische Alternative RT GmbH`.
While the Protocol overview is given here, THE PROTOCOL ITSELF IS EXPLICITLY NOT COVERED BY THIS REPOSITORIES LICENSE.

## The Protocol - Overview
A CoE packet consists of a 4-byte Header and up to 31 8-byte Payload fields.
### The Header
Consists of these values, in order, starting from the 1st byte of the packet.
1. Major Version as unsigned 8-bit integer.
2. Minor Version as unsigned 8-bit integer.
3. Packet length in byte as unsigned 8-bit integer.
4. Packet length in number of payloads as unsigned 8-bit integer.

There is no padding between the header and the first payload field.
### A Payload field
Consists of these values, in order, starting from the 1st byte of the Payload.
1. The CAN-ID of the virtual node which the CMI will create to send the value onto the CAN-Bus. Unsigned 8-bit integer.
    - allowed values are 1-62
2. The `output index` of the transmitted Value. Unsigned 8-bit integer.
    - allowed values are 0-63
    - NOTE: The CMI web-gui shows these offset by 1.
    - e.g.: 2 on-wire == 3 in CMI-Web-GUI.
    - `coe-rs` DOES NOT add this offset. The calling application is responsible for handling the offset if required.
3. Unsigned 8-bit integer. Either:
    - 0: The value is a single bool, in the 8th bit of field 5. (i.e. least significant bit in the first byte)
    - 1: The value is a signed, 32-bit, little-endian integer, stored in field 5.
4. The Unit ID of the value. This determines what phyiscal unit the value corresponds to.
    - You can find a complete list of unit ids in [`AnalogueCOEValue`](crate::AnalogueCOEValue)
    - Note that this ID also determines the decimal places to which the value is stored.
    - e.g.: The unit specifies 3 decimal places (Thousands) => a value of 10 on-wire corresponds to 0.010 in the appropriate unit.
5. The value. Always 4-byte long. Either:
    - a bool in the 8th bit, 0-bits elsewhere
    - a signed, 32-bit, little-endian integer

# Limitations and Stability
`coe-rs` in its current state is (apart from potential bugs I have not found yet) fully compliant to the CoEv2.0 Spec.
CoEv1 is not currently implemented. If you need that protocol, consider opening a PR.

## SemVer pre-1.0
I promise the following SemVer while pre-1.0:
- breaking changes WILL bump the minor version
- minor changed WILL bump the patch version and MAY bump the minor version if they are substantial

## MSRV
Minimum supported Rust version is `rustc 1.80.1`. Earlier versions of rustc may work, but they have not been tested.

# License
This project is licensed under MIT-0 (MIT No Attribution).
By contributing to this repositry, you agree that your code will be licensed as MIT-0.

For my rationale for using MIT-0 instead of another more common license, please see
https://copy.church/objections/attribution/#why-not-require-attribution .

