use std::{
    env,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct Alert(pub Option<(String, u32)>, pub String);
#[derive(Debug)]
pub struct Error(pub Alert);
#[derive(Debug)]
pub struct Warning(pub Alert);

impl Display for Alert {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some((file, line)) = &self.0 {
            write!(f, "{}.{}: ", file, line)?;
        }
        write!(f, "{}", self.1)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1b[0;31merror\x1b[0m: {}", self.0)
    }
}
impl Display for Warning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1b[0;33mwarning\x1b[0m: {}", self.0)
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! make_alert {
    ($($file:expr, $line:expr)? => $($arg:tt)*) => {
        {
            let mut _fl = std::option::Option::None;
            $(
                _fl = std::option::Option::Some(($file.to_string(), $line));
             )?
            $crate::errors::Alert(_fl, format_args!($($arg)*).to_string())

        }
    }
}
#[macro_export]
macro_rules! err {
    ($($file:expr, $line:expr)? => $($arg:tt)*) => {
        {
            $crate::errors::Error($crate::make_alert!($($file, $line)? => $($arg)*))
        }
    }
}
#[macro_export]
macro_rules! error {
    ($($file:expr, $line:expr)? => $($arg:tt)*) => {
        {
            std::result::Result::Err($crate::err!($($file, $line)? => $($arg)*))
        }
    }
}
#[macro_export]
macro_rules! warn {
    ($file:expr, $line:expr => $($arg:tt)*) => {
        {
            eprintln!("{}", $crate::errors::Warning($crate::make_alert!($file, $line => $($arg)*)));
        }
    }
}
