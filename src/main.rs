//#![feature(backtrace)]
//use std::backtrace::Backtrace;
use log::{error, warn, info, debug, trace};
use crate::shared::event_logging::Msgs;

pub mod shared;

fn main() {
    println!("Hello, world!");
    
    // Initialize log4rs.
    log4rs::init_file("resources/log4rs.yml", Default::default()).unwrap();

    // Log each type of message.
    error!("msg1");
    warn!("msg2");
    info!("msg3");
    debug!("msg4");
    trace!("msg5"); 

    // Log standardized messages defined in message module.
    warn!("{}", Msgs::FILE_NOT_FOUND("mydir/myfile.txt"));
    error!("{}", Msgs::DB_CONNECTION_ERROR("mydb", "admin@//mydb.com:5432"));

    // println!("Custom backtrace: {}", Backtrace::force_capture());
}
