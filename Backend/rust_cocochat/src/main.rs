mod log;

use std::{net::{TcpListener,TcpStream},io::*};
use log::*;

fn main() {
    println!("Hello, world!");
    println!("Build was Successfull");

    let server = start_server().unwrap();

}

fn start_server() -> Result<TcpListener> {
    let listner = TcpListener::bind("0.0.0.0:6778");

    let test = Log {
        message: stringify!("test").to_string(),
    };
    test.print();
    return listner;
}
