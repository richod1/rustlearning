use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to the guess game");

    

    let secret_number=rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess");
    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    // let guess :u32=guess.trim().parse().expect("Please input a number");
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    println!("you guessed :{guess}");

    match guess.cmp(&secret_number){
        Ordering::Less=>println!("too small"),
        Ordering::Greater=>println!("too big"),
        Ordering::Equal=>{
            println!("You win!");
            break;
        }
    }

    println!("The secret number is :{secret_number}");
}

    
}