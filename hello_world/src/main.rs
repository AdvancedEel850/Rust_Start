fn main() {
    println!("Hello, world!");
    let number: i32 = 24;
    launch(number);
}

fn launch(number: i32){

    let mut countdown = number;
    
    if countdown != 3{
        countdown = 3;
    }

    while countdown != 0{
        println!("{countdown}!");
        countdown -=1 ;
    }
    println!("Launch!");
}
