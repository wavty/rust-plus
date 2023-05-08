use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, author="wavty", about="httpie", long_about=None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, required(false))]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
