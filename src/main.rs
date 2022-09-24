use clap::Parser;
use trabalho::lexical::parse_code;

/// Basic parser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    println!("Parsing {}", &args.filename);
    parse_code(&args.filename);
}
