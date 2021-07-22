mod k8;
mod errors;

use std::process::exit;
pub use errors::{Result, Error};

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
