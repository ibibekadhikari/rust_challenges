
//Creating Modules allows us to partition our code and make it in a small pieces. Pub is to give public access. 
//We can create multiple mod, and it's given pub so that it can be used in other file like main.rs too.
pub mod inout{
    //Using standard input outpur crate.
    use std::io;
    pub fn input_int() -> i64 {
        let mut my_string = String::new();
        io::stdin().read_line(&mut my_string).expect("Please check the given input");
        let intvalue : i64 = my_string
                             .trim() //To ignore white spaces.
                             .parse() //To parse into an integer.
                             .expect("Please check the given input.");  //To handle error if any occurs.
        intvalue
    }
}