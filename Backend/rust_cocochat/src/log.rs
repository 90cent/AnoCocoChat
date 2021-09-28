/*

use std::{io::*};
use helper::*;


pub fn log(message: &str) {

}

pub fn log_result<T>(result: std::io::Result<T>){
    let res = match result {
        Ok(_) => println!("[{} - {}] | Status: {} | {}",get_date(),get_time(),"Function execution was succsessfull","OK"),
        Err(k) => eprintln!("[{} - {}] | Status: {} | {}",get_date(),get_time(),"Function execution was NOT succsessfull",k)
    };

    // To Do... A good error handling system for Result or smthing lol
}
