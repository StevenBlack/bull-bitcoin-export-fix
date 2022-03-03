extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::fmt;
use serde::Deserialize;
use std::io;


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

// type Record = (i32, f64, String, f64, String, String, String, String);

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "number")]
    number: i32,
    #[serde(rename = "from-amount")]
    from_amount: f64,
    #[serde(rename = "from-currency")]
    from_currency: String,
    #[serde(rename = "to-amount")]
    to_amount: f64,
    #[serde(rename = "to-currency")]
    to_currency: String,
    #[serde(rename = "payment-method")]
    payment_method: String,
    #[serde(rename = "created-at")]
    created_at: String,
    #[serde(rename = "completed-at")]
    completed_at: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // "{}, {}, \"{}\", {}, \"{}\", \"{}\", \"{}\", \"{}\"",
        write!(f,
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.number,
            self.from_amount,
            self.from_currency,
            self.to_amount,
            self.to_currency,
            self.payment_method,
            self.created_at,
            self.completed_at
        )
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .double_quote(false)
        .from_path(file_path)?;

    let mut wtr = csv::Writer::from_writer(io::stdout());

    // the header line
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        wtr.write_record(headers)?;
        wtr.flush()?;
        // println!("{:?}", headers);
    }

    // the data
    for result in rdr.deserialize() {
        let record: Record = result?;
        // wtr.write_record(record)?;
        println!("{}", record);
    }
    // wtr.flush()?;
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