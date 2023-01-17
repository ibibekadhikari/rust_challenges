//Declaring Const in the screaming snake case.
const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;
//Main function begins.
fn main() {
	//using pattern to declare variable and mut defines mutability.
    	let (mut missiles,ready) = (STARTING_MISSILES , READY_AMOUNT);
	println!("{STARTING_MISSILES}");
	println!("Firing {} of my {} missiles...", ready, missiles);
	missiles = missiles -  ready;
	println!("{} missiles left", missiles);
}

