#![allow(async_fn_in_trait)]

use clap::Parser;

mod cli;
mod connectivity;

fn main() {
    let cli = cli::Cli::parse();
    println!("Hello, world!");
}
