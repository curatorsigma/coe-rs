use crate::{Packet, Payload};

pub struct PacketIterator<'a> {
    packet: &'a Packet,
    idx: usize,
}
impl<'a> PacketIterator<'a> {
    pub fn new(packet: &'a Packet) -> Self {
        Self {
            packet,
            idx: 0,
        }
    }
}
impl<'a> Iterator for PacketIterator<'a> {
    type Item = &'a Payload;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.packet.len() {
            let res = Some(&self.packet.payloads[self.idx]);
            self.idx += 1;
            res
        } else {
            None
        }
    }
}
