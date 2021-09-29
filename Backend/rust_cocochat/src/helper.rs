use chrono::{DateTime,Date,Utc,Local,Datelike,Timelike};
use std::u8::*;

pub fn get_date() -> String {
    let cur_time = Local::now();
    return format!("{}.{}.{}",cur_time.date().day(),cur_time.date().month(),cur_time.date().year());
}

pub fn get_time() -> String {
    let cur_time = Local::now();
    return format!("{}:{}:{}",cur_time.time().hour(),cur_time.time().minute(),cur_time.time().second());
}

pub fn NewPacket(packet_arg: &mut Vec<u8>) -> packet::BytePacket {
    let p = packet_arg.clone();
    let packet_ = packet::BytePacket {
        byte_vector: p
    };

    return packet_;
}

pub mod packet {
    pub struct BytePacket {
       pub byte_vector: Vec<u8>
    }

    pub trait ToStringify {
        fn StringTable(&self) -> String;
    }

    impl ToStringify for BytePacket {
        fn StringTable(&self) -> String {
            let table = format!("{:02x?}",&self.byte_vector);

            return table;
        }
    }
}
