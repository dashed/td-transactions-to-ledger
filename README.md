td-transactions-to-ledger
=========================

> Convert TD Bank transactions to ledger-cli compatible transactions

Usage
=====

```
td-transactions-to-ledger 0.1.0
Alberto Leal <mailforalberto@gmail.com>
Convert TD Bank transactions to ledger compatible transactions

USAGE:
    td-transactions-to-ledger [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --account <account_name>    Sets account for each transaction
    -n, --num <first_n>             Only show last N transactions.

ARGS:
    <INPUT>    Sets the input CSV file to use
```

1. Export transactions from TD Bank into csv format.

2. `td-transactions-to-ledger accountactivity.csv -n 10 | pbcopy`

3. Paste transactions into your ledger-cli file.

Install
=======

## From source

- Install rust. (e.g. set via https://www.rustup.rs/)
- `git clone git@github.com:dashed/td-transactions-to-ledger.git`
- `cargo build --release`
- `cp target/release/td-transactions-to-ledger /usr/local/bin` (or similar)

License
=======

MIT.
