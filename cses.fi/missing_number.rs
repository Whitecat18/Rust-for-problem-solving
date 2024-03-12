use std::io;
 
fn main(){
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).unwrap();
    let leng = string1.trim().parse::<u64>().expect("Error");
 
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Error");
 
    let inp: Vec<u64> = string.split_whitespace()
        .map(|s| s.parse().expect("plx provide proper message"))
        .collect();
 
    let seq: u64 = inp.iter().sum();
    let n = leng;
    let exp = n * (n+1) / 2;
    let missing_nums = exp - seq;
 
    println!("{}",missing_nums);
}
