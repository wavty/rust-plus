mod cmd;
use clap::Parser;

fn main() {
    let args = cmd::Opts::parse();
    println!("{:?}", args);
}
