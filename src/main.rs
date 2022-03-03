extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;


// [
//     "number",
//     "from-amount",
//     "from-currency",
//     "to-amount",
//     "to-currency",
//     "payment-method",
//     "created-at",
//     "completed-at"
// ]
type Record = (i32, f64, String, f64, String, String, String, String);

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .double_quote(false)
        .from_path(file_path)?;

    // the header line
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }

    // the data
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}