use add_sub::add;
use add_sub::subtract;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    x: i32,

    y: i32,
}

fn main() {
    let args = Args::parse();
    println!("Addition result: {}", add(&args.x, &args.y));
    println!("Subtraction result: {}", subtract(&args.x, &args.y));
}
