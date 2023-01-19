//Just to get an insight ideas on how mod and their access/stucture works.
//We have inout_dir directory inside src so using that directory name with keyword mod.
mod inout_dir;
fn main() {
    println!("Enter a number: ");
    //Here comes the magic, so we have used inout directory and then inout mod and then input_int as a function for our integer easy.
    let x : i64 = inout_dir::inout::input_int();
    println!("You have entered an integer value of {}",x);
}
