use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("hello, and welcome to my guessing game");
    println!("lets begin, do you want to play? yes/no");

    // let answer_yes = "yes";
    let mut game_terms = String::new();
    io::stdin().read_line(&mut game_terms).expect("Failed to read line");


    // match string input to fit
    if game_terms.trim() == "yes"{ // trim clears the new line created on input
        println!("welcome, let's start");
    } else{
        println!("okay, come back next time");

        // close program
    }

    loop{
        println!("please guess a number 1-100");
        let number = rand::thread_rng().gen_range(1..=100);
        println!("{}", number);
        let mut guess_number = String::new();
        io::stdin().read_line(&mut guess_number).expect("Failed to read line");

        let guess_number: u32 = guess_number.trim().parse().expect("input a number");
    
        match number.cmp(&guess_number){
            Ordering::Less => println!("oops, too small"),
            Ordering::Greater => println!("oops, too big"),
            Ordering::Equal =>{
                println!("you win");
                break;
            } 
        }
    }

    // you can use this foundation to build a simple word game


    
    // match game_terms.as_str(){
    //     "yes" => println!("welcome"),
    //     "no" => println!("okay, try again"),
    //     _=>println!("great, let's go")
    // }
}
