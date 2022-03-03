extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::fmt;
use serde::Deserialize;
use std::io;

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
        write!(f,
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.number,
            self.from_amount,
            self.from_currency,
            self.to_amount,
            self.to_currency,
            self.payment_method,
            date_fix(&self.created_at),
            date_fix(&self.completed_at),
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

fn date_fix(d: &str ) -> String {
    // d looks like this:
    // "Sun Dec 13 2020 00:00:01 GMT+0000 (UTC)"
    let mo = match &d[4..7] {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => "99"
    };
    format!("{}-{}-{}", d[11..15].to_string(), mo, d[8..10].to_string())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_fix() {
        assert_eq!(date_fix("Sun Dec 13 2020 00:00:01 GMT+0000 (UTC)"), "2020-12-13".to_string() );
        assert_eq!(date_fix("Tue Mar 02 2021 02:33:43 GMT+0000 (UTC)"), "2021-03-02".to_string() );
    }
}
