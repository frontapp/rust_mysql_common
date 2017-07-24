extern crate atoi;
extern crate bit_vec;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[cfg(feature = "rustc_serialize")]
pub extern crate rustc_serialize;
#[cfg(not(feature = "rustc_serialize"))]
pub extern crate serde;
#[cfg(not(feature = "rustc_serialize"))]
pub extern crate serde_json;
extern crate smallvec;
extern crate time;
extern crate twox_hash;
pub extern crate uuid;

/// Macro to conveniently generate named parameters for a statement.
/// Parameter name is T where String: From<T>, and 
///
/// ```ignore
/// params! {
///     "param_name_1" => param_value_1,
///     "param_name_2" => param_value_2,
/// }
/// ```
#[macro_export]
macro_rules! params {
    ($($name:expr => $value:expr),*) => (
        vec![
            $((::std::string::String::from($name), $crate::value::Value::from($value))),*
        ]
    );
    ($($name:expr => $value:expr),*,) => (
        params!($($name => $value),*)
    );
}

macro_rules! split_at_or_err {
    ($reader:expr, $at:expr, $msg:expr) => {
        if $reader.len() >= $at {
            Ok($reader.split_at($at))
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, $msg))
        }
    };
}

#[macro_export]
macro_rules! read_lenenc_str {
    ($reader:expr) => {
        $reader.read_lenenc_int().and_then(|len| {
            let (value, rest) = split_at_or_err!($reader,
                                                 len as usize,
                                                 "EOF while reading length-encoded string")?;
            *$reader = rest;
            Ok(value)
        })
    };
}

pub mod constants;
pub mod io;
pub mod named_params;
#[macro_use]
pub mod packets;
pub mod params;
pub mod row;
pub mod value;