use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut begin: u32 = 1;
    let mut end: u32 = 100;
    let mut count: u32 = 0;

    loop {
        count += 1;
        print!("Please input your guess[{begin}-{end}]:");
        io::stdout().flush().expect("");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read guess");

        let guess_number: u32 /* Type */ /* Type */ = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guess is:{guess_number}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => {
                begin = guess_number+1;
                println!("{guess_number} is too SMALL");
                io::stdout().flush().expect("");
            }
            Ordering::Greater => {
                end = guess_number-1;
                println!("{guess_number} is too BIG");
                io::stdout().flush().expect("");
            }
            Ordering::Equal => {
                println!("Bingo! You win after try {count} times.");
                break;
            }
        }
    }
}
