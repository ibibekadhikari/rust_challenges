fn linear_search(x:i32) -> bool 
{
    //Defining an array with type i32 and sizeof 10
    let my_arr:[i32;10] = [3,4,5,1,2,6,8,7,9,10];
    let mut found: bool = false;
    //Running a forloop in an array.

    for i in my_arr{
        if x == i{
        found = true;
        }
    }
    found
}
fn main() {
    let searched_value = linear_search(8);
    if searched_value{
        println!("The value is found in the array.");
    }else{
        println!("The value is not found in the array.");
    }
}
