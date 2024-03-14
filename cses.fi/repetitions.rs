fn longest(dna: String) -> u64{
    let (mut prev, mut max, mut repetation) = (' ',0,0);

    for c in dna.chars(){
        if c == prev{
            repetation += 1;
        } else{
            max = max.max(repetation);
            repetation = 1;
            prev = c;
        }
    }
    max.max(repetation)
}

macro_rules! read {
    ($out:ident) => {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Validate the input");
        let $out = input;
    };
}

fn main(){
    read!(input);
    println!("{}",longest(input.trim().to_string()));
}
