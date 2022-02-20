use heapless::Vec;

use crate::packet::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;

use super::packet_type::PacketType;
use super::property::Property;

pub const MAX_PROPERTIES: usize = 18;

pub struct ConnackPacket<'a> {
    // 7 - 4 mqtt control packet type, 3-0 flagy
    pub fixed_header: u8,
    // 1 - 4 B lenght of variable header + len of payload
    pub remain_len: u32,
    pub ack_flags: u8,
    pub connect_reason_code: u8,
    pub property_len: u32,
    pub properties: Vec<Property<'a>, MAX_PROPERTIES>,
}

impl<'a> ConnackPacket<'a> {
    pub fn decode_connack_packet(&mut self, buff_reader: &mut BuffReader<'a>) {
        if self.decode_fixed_header(buff_reader) != (PacketType::Connack).into() {
            log::error!("Packet you are trying to decode is not CONNACK packet!");
            return;
        }
        self.ack_flags = buff_reader.read_u8().unwrap();
        self.connect_reason_code = buff_reader.read_u8().unwrap();
        self.decode_properties(buff_reader);
    }
}

impl<'a> Packet<'a> for ConnackPacket<'a> {
    fn encode(&mut self, buffer: &mut [u8]) {}
    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) {
        self.decode_connack_packet(buff_reader);
    }

    fn set_property_len(&mut self, value: u32) {
        self.property_len = value;
    }

    fn get_property_len(&mut self) -> u32 {
        return self.property_len;
    }

    fn push_to_properties(&mut self, property: Property<'a>) {
        self.properties.push(property);
    }

    fn set_fixed_header(& mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(& mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
