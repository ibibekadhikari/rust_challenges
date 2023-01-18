//This program is here inorder to learn how input works in Rust.
//Import standard input/output library (Crate in Rust) for input like in C hahaha.

//We use "use" keyword in order to implemet library in the program.
use std::io;
fn main() {
    println!("Enter a name: ");
    let mut input_number = String::new();
    let mut guess = String::new();
    //Here using the stdin() from std::io crate. It's just std is the crate and io is the mod and stdin().read_line() is function.
    // io::stdin().read_line(&mut guess);  <- This code work fine but shows warnings.
    //The above code works fine to accept input but shows warning saying it should handle error if occurs So can be formatted as.
    io::stdin().read_line(&mut guess).expect("failed to readline");
    //The above command is used for reading and expect is used to handle error.
    println!("Your Guess is {}", guess);

    //FOR NUMBER IT IS TRICKY.
    println!("Enter an integer: ");
    io::stdin() // the rough equivalent of `std::cin`
    .read_line(&mut input_number) // actually read the line
    .expect("Failed to read line"); // which can fail, however
    let x: i32 = input_number
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer"); // which, again, can fail
    println!("The number input is {} and it is integer so its square is {}", x, (x*x));
}
