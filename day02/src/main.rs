mod part1;
mod part2;

use part1::part01;
use part2::part02;
fn main() {
    let path = "day02/data.txt";
    println!("{}", part01(path));
    println!("{}", part02(path));
}
