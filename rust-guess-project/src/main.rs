use std::io;
fn main() {

    println!("hello, and welcome to my guessing game");
    println!("lets begin, do you want to play?");

    // let answer_yes = "yes";
    let mut game_terms = String::new();
    io::stdin().read_line(&mut game_terms).expect("Failed to read line");


    // match string input to fit
    if game_terms.trim() == "yes"{ // trim clears the new line created on input
        println!("welcome, let's start");
    } else{
        println!("okay, come back next time")
    }

    // you can use this foundation to build a simple word game


    
    // match game_terms.as_str(){
    //     "yes" => println!("welcome"),
    //     "no" => println!("okay, try again"),
    //     _=>println!("great, let's go")
    // }
}
