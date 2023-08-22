// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Sparkling,
    Sweet,
    Fruity,
}
struct Drink {
    flavor: Flavour,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavour::Sparkling => println!("sparkling"),
        Flavour::Sweet => println!("sweet"),
        Flavour::Fruity => println!("fruity"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}
fn main() {
    let sweet = Drink {
        flavor: Flavour::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);
    let fruity = Drink {
        flavor: Flavour::Fruity,
        fluid_oz: 10.0,
    };
    print_drink(fruity);
    let sparkling = Drink {
        flavor: Flavour::Fruity,
        fluid_oz: 10.0,
    };
    print_drink(sparkling);
}
