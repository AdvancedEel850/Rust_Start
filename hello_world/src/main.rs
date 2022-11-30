use std::io;
use rand::Rng;

fn main() {

    println!("Would you like to run the test? (y/n) ");

    let mut testing :String = String::new();

    io::stdin()
        .read_line(&mut testing)
        .expect("Failed to read line");

    testing = testing.trim_end().to_string();

    if testing == "y"{
        println!("Hello, world!");
        let number: i32 = 24;
        launch(number);

        let x: i32 = 0;
        let y :i32 = 5;

        adding(x, y)
    }
    else{
        game()
    }
}

fn launch(mut number: i32){

    
    if number != 5{
        number = 5;
    }

    while number != 0{
        println!("{number}!");
        number -=1 ;
    }
    println!("Launch!");
}

fn adding(mut x : i32, y : i32){
    println!("\nCurrent values:\nx, {x} y, {y}");
    println!("\nWould you like to change your x? (y/n)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    guess = guess.trim_end().to_string();

    if guess.eq(&String::from("y")){

        let mut new_x : String = String::new();
        println!("\nWhat number would you like x to be? ");

        io::stdin()
        .read_line(&mut new_x)
        .expect("Failed to read line");

        let new_x: i32 = new_x.trim().parse().expect("Please type a number!");

        x = new_x;
    }

    println!("\n{} + {} = {}", x, y, x+y)
}

fn game(){

    println!("Welcome to my guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses: i32 = 0;
    
    while guesses < 5{
        
        println!("What number would you like to guess: ");
        let mut user_guess : String = String::new();

        io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

        let user_guess: i128 = user_guess.trim().trim_end().parse().expect("Please type a number!");

        if user_guess == secret_number{
            println!("You got it! The secret number was {}", secret_number);
            break;
        }
        else{
            println!("Not the right number!");
            guesses += 1;
        }
    }

    if guesses >= 5{
        println!("Too bad, better luck next time!")
    }

    println!("Would you like to play again? (y/n) ");

    let mut again:String = String::new();

    io::stdin()
        .read_line(&mut again)
        .expect("Failed to read line");
    
    again = again.trim_end().to_string();

    if again.eq(&String::from("y")){
        game()
    }
    

}