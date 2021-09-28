mod helper;

use std::{net::{TcpListener,TcpStream},io::*,time::*};
use helper::*;


fn main() {
    println!("Hello, world!");
    println!("Build was Successfull");

    let server = start_server();
    
}

fn start_server() -> TcpListener {
    let listner = TcpListener::bind("0.0.0.0:6778");
    
    return listner.unwrap();
}
