fn main() {
    println!("Hello, world!");
    another_function();
    println!("BYE!");
    another_function_two();
    println!("HI AGAIN!");
    println!("BACK TO MAIN");

}

fn another_function(){
    println!("Entering function one! ");
    another_function_two();
}

fn another_function_two(){
    println!("Entering function two! ");
}
