use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let mut input: String = String::new(); // new empty mutable string
        println!("BOMB GONNA BLAST IN ?");
        io::stdin().read_line(&mut input).expect("Invalid Input"); // getting the string input

        let timer: u16 = match input.trim().parse() {
            Ok(timer) => timer,
            Err(_) => {
                println!("input is not an integer");
                continue;
            }
        };

        start_timer(timer);
        println!("Allah Hu Akhbar 🔥💣");
        break;
    }
}
fn start_timer(time: u16) {
    for i in (1..=time).rev() {
        println!("Time left {} seconds", i);
        sleep(Duration::from_secs(1));
    }
}
