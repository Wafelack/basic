mod errors;
mod k8;

pub use errors::{Error, Result};
use std::process::exit;

fn try_main() -> Result<()> {
    warn!(file!().to_string(), line!() => "foobar");
    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => exit(0),
        Err(e) => eprintln!("{}", e),
    }
    exit(1);
}
