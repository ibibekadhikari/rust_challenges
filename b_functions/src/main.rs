fn main() {
	//creating a Normal function.
	//Write it in an snake case.
	
	fn print_hello_world(){
	println!("Hello World!");
	}
	//Defining a func which takes i64 inputs and return i64 inputs that is why we have used -> i64 like in solidity.

	fn sum(x: i64, y: i64) -> i64 {
	let c : i64 = x + y;
	//We don't need to write return in order to return the value from the functions.
	c
	}

	//Float function.
   	fn multiply(a:f64, b:f64) -> f64  {
	a*b
	}
	print_hello_world();
	println!("The Sum is {}",sum(12,12));
	//Use (pass) the float literal for the float parameters.
	println!("The multiply is {}", multiply(10.0,10.0));
}
