# 🙄 bull-bitcoin-export-fix

[Bull Bitcoin](https://bullbitcoin.com/) is a Canadian non-custodial bitcoin exchange. I used to be a customer, but abandoned the service because personal record-keeping and tax compliance is a pain.
## Motivation

Bull Bitcoin offers a comically inflexible "Export my orders" button that gives you a so-called `.csv` file that has intractable problems.

Bull Bitcoin support has declined written suggestions to make their data more interoperative.

This repo contains the code I use to transform the garbage data into a format I can easily use in a spreadsheet.  In other words, this repo takes shitty data and makes a usable `.csv` file I can actually use.

### Specific problems with the Bull Bitcoin CSV files

In short,
* It's not a `.csv` file according to the [CSV, Comma Separated Values (RFC 4180) spec](https://datatracker.ietf.org/doc/html/rfc4180).
* It's not a `.csv` file that's anything like [what you might expect](https://en.wikipedia.org/wiki/Comma-separated_values).
* The Bull Bitcoin `.csv` file's timestamp columns aren't [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) compliant, so parsing dates from the data is a pain in the ass.
* Basically the Bull Bitcoin `.csv` file is a minimum effort, give-zero-shits text dump from a database upon which they've slapped a `.csv` file extension.  If you're familiar with PostgreSQL, think [`pg_dump`](https://www.postgresql.org/docs/14/app-pgdump.html) — that's what they effectively give you, from whatever database Bull Bitcoin uses.

**Therefore**
* The Bull Bitcoin `.csv` file can't be directly imported to any standard spreadsheet or database, and everything about this raw data is a pain.

## Invocation (for now)

This is written in Rust.  At this stage you can clone this repo and call it with `cargo run` or with `cargo build` and invoking the binary from the `./target/debug/` folder.

**Coming soon**:
* Convert timestamp values to ISO 8601 format.
* A proper binary you can invoke conventionally from your MacOS, Linux, or Windows OS.

### Calling with `cargo run`

This presumes the so-called "`.csv`" file is named `bull-bitcoin-orders.csv` located in the local `~/Downloads` folder.

```
cargo run -- ~/Downloads/bull-bitcoin-orders.csv
```
### Calling the debug binary:

This presumes the so-called "`.csv`" file is named `bull-bitcoin-orders.csv` located in the local `~/Downloads` folder.

```
./target/debug/bull-bitcoin-export-fix ~/Downloads/bull-bitcoin-orders.csv
```