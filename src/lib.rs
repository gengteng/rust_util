#[macro_use]
extern crate lazy_static;
extern crate term;

use std::io::{ Error, ErrorKind, };

pub mod util_io;
pub mod util_os;
pub mod util_env;
pub mod util_cmd;
pub mod util_msg;
pub mod util_str;
pub mod util_size;
pub mod util_file;
pub mod util_time;

/// iff!(condition, result_when_true, result_when_false)
#[macro_export]
macro_rules! iff {
    ($c:expr, $t:expr, $f:expr) => {
        if $c { $t } else { $f }
    };
}

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn new_box_error(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
}

pub fn new_box_ioerror(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
}
