use chrono::{DateTime,Date,Utc,Local,Datelike,Timelike};
use std::u8::*; 
use json::*;
use crate::log::{elog,log};

pub fn get_date() -> String {
    let cur_time = Local::now();
    return format!("{}.{}.{}",cur_time.date().day(),cur_time.date().month(),cur_time.date().year());
}

pub fn get_time() -> String {
    let cur_time = Local::now();
    return format!("{}:{}:{}",cur_time.time().hour(),cur_time.time().minute(),cur_time.time().second());
}


// packet-system
pub fn NewPacket(packet_arg: &mut Vec<u8>) -> packet::BytePacket {
    let p = packet_arg.clone();
    let packet_ = packet::BytePacket {
        byte_vector: p
    };

    return packet_;
}

pub fn NewBuildedPacket(recieved_packet_client: packet::BytePacket) {
    let bytes = recieved_packet_client.byte_vector;
    let StringFromBytes = String::from_utf8(bytes);
    
    log(StringFromBytes);
}

pub mod packet {

    pub mod message {
        pub struct MessageBuild {
            pub message: String,
            pub message_lenght: i32
        }
    }

    pub struct BytePacket {
       pub byte_vector: Vec<u8>
    }

    pub struct BuildedPacket {
        pub bytes: Vec<u8>,
        pub client_token: String,
        pub client_pwhash: String,
        pub client_message: message::MessageBuild,
        pub client_anonumber: i32
    }

    pub trait ToStringify {
        fn StringTable(&self) -> String;
        fn ParsePacket(&self) -> Result<String,Error>;      //returns ip address of user so the server knows where to send shit
    }

    impl ToStringify for BytePacket {
        fn StringTable(&self) -> String {
            let table = format!("{:02x?}",&self.byte_vector);

            return table;
        }

        fn ParsePacket(&self) {
            
        }
    }
}
