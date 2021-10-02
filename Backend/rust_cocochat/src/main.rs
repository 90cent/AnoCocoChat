pub mod helper;
pub mod log;

use std::{io::*, net::{TcpListener,TcpStream}, os::*, *, self, vec::*};
use local_ipaddress::*;
use helper::{*,packet::ToStringify,packet::BytePacket};

use log::*;

fn main() {
    let mut connected_clients = 0;
    println!("Hello, world!");
    println!("Build was Successfull");

    let server = start_server();
    for client_stream in server.incoming() {
        match client_stream {
            Ok(stream) => {
                connected_clients += &1 * &4;
                let client_handle = thread::spawn(move || {
                    log(format!("{:#?}.Client Connected",&connected_clients));
                    chat_client(stream);
                });
                // Create Client Handle in seperate Threads
                let handle = match client_handle.join() {
                    Ok(hndl) => log(String::from("Ok")),
                    Err(_) => log(String::from("Handle error")),
                };
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
    
    log(format!("Packet: Byte:{:#?} String:{}",size1,"--"));
}

fn start_server() -> TcpListener {
    let ip_address = &format!("{}:6778",local_ipaddress::get().unwrap().as_str());
    let listner = TcpListener::bind(ip_address);

    log(format!("Server running on {}", ip_address));

    return listner.unwrap();
}
