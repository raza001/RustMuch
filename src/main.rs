use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {

    println!("Guess the number");

    let secret_value=rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please input your Guess");

        let mut guess:String =String::new();
    
        io::stdin().read_line(&mut guess).expect("ye chalna");
    
        let guess:u32 =match guess.trim().parse()  {
            Ok((num))=>num,
            Err(_)=> continue,
    
        };
    
        match guess.cmp(&secret_value) {
            Ordering::Less=>println!("too short"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("big"),
        }
    }

}
