use std::io;

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

