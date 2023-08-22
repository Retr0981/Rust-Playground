// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Red,
    Yellow,
    Pink,
}

fn print_color(my_color: Colour) {
    match my_color {
        Colour::Red => println!("It's Red"),
        Colour::Yellow => println!("It's Yellow"),
        Colour::Pink => println!("It's Pink"),
    }
}
fn main() {
    print_color(Colour::Red);
}
