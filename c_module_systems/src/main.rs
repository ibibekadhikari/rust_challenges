
//Use keywords helps us to import the function from lib.rs which is inside src.
//Here the first name before two colon is projectname and then function.s
use c_module_systems::greet;
fn main() {
    greet();
    //Or we can simply don't use USE and use with the project name but this could be tideous to work it but work same.
    c_module_systems::greet();
}
