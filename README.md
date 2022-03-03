# 🙄 bull-bitcoin-export-fix

Fixing Bull Bitcoin's comically awful csv exports


**TAKE NOTE:** 🎆 work in progress 🎆

## Invocation (for now)

### Calling with `cargo run`

This presumes the so-called "`.csv`" file is named `bull-bitcoin-orders.csv` in the local `~/Downloads` folder.

```
cargo run -- cargo run -- ~/Downloads/bull-bitcoin-orders.csv
```
### Calling the debug binary:

```
./target/debug/bull-bitcoin-export-fix ~/Downloads/bull-bitcoin-orders.csv
```