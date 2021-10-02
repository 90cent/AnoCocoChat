use crate::helper::*;
use std::{io::*};

pub fn log(message: String) {
    println!("[{} - {}] | Status: {} | {}",get_date(),get_time(),&message,"OK");
}

pub fn elog(message: String){
    eprintln!("[{} - {}] | Status: {} | {}",get_date(),get_time(),"Function execution was NOT succsessfull",&message);
}