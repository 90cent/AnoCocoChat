use chrono::{DateTime,Date,Utc,Local,Datelike,Timelike};

pub fn get_date() -> String {
    let cur_time = Local::now();
    return format!("{}.{}.{}",cur_time.date().day(),cur_time.date().month(),cur_time.date().year());
}

pub fn get_time() -> String {
    let cur_time = Local::now();
    return format!("{}:{}:{}",cur_time.time().hour(),cur_time.time().minute(),cur_time.time().second());
}