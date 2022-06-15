#![allow(unused)]

use clap::Parser;
use tohang_lib;
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
struct Cli {
    /// Adds a new item to do
    #[clap(long, short, value_parser)]
    add: Option<String>,

    /// Ticks off a completed task
    #[clap(long, short, value_parser)]
    complete: Option<String>,

    /// Generates a fun thing to do from the list
    #[clap(long, short, action)]
    do_something: bool,

    /// Lists all incomplete stuff to do
    #[clap(long, short, action)]
    list: bool,

    /// Lists all stuff done
    #[clap(long, action)]
    list_complete: bool,
}

fn main() {
    // TODO fix relative url
    let DATABASE_URL = "sqlite:db.db";
    env::set_var(DATABASE_URL, "1");

    let cli = Cli::parse();
}
