// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Assignment {
    locker: Option<i32>,
    name: String,
}
fn main() {
    let student = Assignment {
        locker: Some(12),
        name: "David".to_owned(),
    };

    match student {
        Some(ans) => println!(""),
    }
}
