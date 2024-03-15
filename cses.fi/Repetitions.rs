use std::io;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");
 
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut array: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();
 
    let mut moves = 0;
    for i in 1..n {
        if array[i] < array[i - 1] {
            moves += array[i - 1] - array[i];
            array[i] = array[i - 1];
        }
    }
 
    println!("{}", moves);
}
