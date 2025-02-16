use std::io;

fn main() {
    let mut input = String::new();
    println!("Gimme the input bitch");
    io::stdin().read_line(&mut input).expect("invalid input"); // string input

    let deci: u16 = match input.trim().parse() {
        Ok(deci) => deci,
        Err(_) => {
            print!("invalid number");
            return;
        }
    };
    let binans = binasryconvert(deci);
    println!("decimal vlaue : {} binary value : {}", deci, binans)
}

fn binasryconvert(deci: u16) -> u128 {
    let mut val = deci;
    let mut strans = String::new();

    while val != 0 {
        if val % 2 == 0 {
            strans.push_str("0");
        } else {
            strans.push_str("1");
        }

        val = val / 2;
    }

    let revstr = strans.chars().rev().collect::<String>();
    // println!("{}", strans);
    let ans: u128 = revstr
        .trim()
        .parse()
        .expect("Failed to parse binary string");

    return ans;
}
