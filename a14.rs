// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}
fn main() {
    let persons = vec![
        Person {
            age: 10,
            name: String::from("David Armah"),
            favorite_color: "red".to_owned(),
        },
        Person {
            age: 8,
            name: String::from("Clement Nii"),
            favorite_color: String::from("Blue"),
        },
        Person {
            age: 9,
            name: String::from("Smart Armah"),
            favorite_color: "yellow".to_owned(),
        },
        Person {
            age: 20,
            name: String::from("Clt Nii"),
            favorite_color: String::from("Pink"),
        },
    ];

    for person in persons {
        if person.age <= 10 {
            print(&person.name);
            print(&person.favorite_color)
        }
    }
}
