#![allow(non_snake_case)]
use ::function_name::named;

/**
Define the application's standard log messages.

The name of the function is the message identifier, which is written as part
of the message following a formatting convention.  The convention expects
the function name as the first string parameter passed into a formatter.

Each log message function defines zero or more parameters to incorporate 
into the log message.
*/
pub struct Msgs {}
impl Msgs {
    #[named]
    pub fn FILE_NOT_FOUND(filename: &str) -> String {
        return format!("{}: {}.", function_name!(), filename);
     }

    #[named]
    pub fn DB_CONNECTION_ERROR(dbname: &str, dburl: &str) -> String {
        return format!("{}: Unable to connect to database '{}' at {}.", function_name!(), dbname, dburl);
    }
}
