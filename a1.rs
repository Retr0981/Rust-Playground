// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    my_name("David", "Armah");
}

fn my_name(first: &str, last: &str) {
    println!("My first name is {:?}", first);
    println!("My last name is {:?}", last);
}
