mod part1;
mod part2;

use part1::part01;
use part2::part02;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the file with data.
    #[arg()]
    data:String,
}
fn main() {
    let args = Args::parse();
    let res1 = part01(&args.data);
    let res2 = part02(&args.data);
    println!("{}", res1);
    println!("{}", res2);
    
}
