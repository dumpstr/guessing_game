use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("your guess wasn't between 1-100, inputted {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("the super secret number is: {}", secret_number);
    loop {
        println!("input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
       
        if guess < 1 || guess > 100 {
            println!("the super secret number is between 1-100");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("u did it!! woohoo!");
                break;
            }
        }
    }
}
