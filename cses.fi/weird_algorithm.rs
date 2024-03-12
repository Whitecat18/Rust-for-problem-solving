use std::io;
fn main(){
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).unwrap();
    let mut leng = string1.trim().parse::<u64>().expect("Error");
    
    print!("{} ", leng);
    while leng != 1{
        if leng % 2 == 0{
            leng /= 2;
            print!("{} ", leng);
        } else{
            leng = 3 * leng  + 1;
            print!("{} ", leng);
 
        }
    }
}
