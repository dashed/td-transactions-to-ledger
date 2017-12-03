// crates

extern crate clap;
extern crate csv;

// 3rd-party imports

use clap::{App, Arg};

fn main() {
    let matches = App::new("td-transactions-to-ledger")
        .version("0.1.0")
        .author("Alberto Leal <mailforalberto@gmail.com>")
        .about("Convert TD Bank transactions to ledger compatible transactions")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    println!("Hello, world!");
}
