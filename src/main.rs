mod part2; 
fn main() {
    let result: usize = part2::solve(include_str!("../input.txt").trim());
    println!("{result}");
}

