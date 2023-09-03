// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_nums = vec![10, 20, 30, 40];

    for num in &my_nums {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("the length of the vec is {:?}", my_nums.len());
}
