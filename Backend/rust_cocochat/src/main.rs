pub mod helper;
pub mod log;

use std::{net::{TcpListener,TcpStream},io::*,os::*,vec::*};
use local_ipaddress::*;
use helper::{*,packet::ToStringify,packet::BytePacket};

use log::*;

fn main() {
    println!("Hello, world!");
    println!("Build was Successfull");

    let server = start_server();
    for client_stream in server.incoming() {
        match client_stream {
            Ok(stream) => {
                log(String::from("Client connected..."));
                chat_client(stream);
            },
            Err(e) => { log(String::from("error")); }
        };
    }
}

fn chat_client(mut client_stream: TcpStream) {
    let mut data: Vec<u8> = vec![0 as u8; 50];

    let size1 = client_stream.read_to_end(&mut data).unwrap();
    //let size2 = client_stream.read_to_string(packet_string).unwrap();  
    

    let packet = helper::NewPacket(&mut data);
    
    log(format!("Recieved Packet: Byte:{} String:{}",packet.StringTable()," -- "));
}

fn start_server() -> TcpListener {
    let ip_address = &format!("{}:6778",local_ipaddress::get().unwrap().as_str());
    let listner = TcpListener::bind(ip_address);

    log(format!("Server running on {}", ip_address));

    return listner.unwrap();
}
