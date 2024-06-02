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

enum Flavors {
    VirginMogito,
    Soda,
    Fruity,
}
struct Drink {
    flavor: Flavors,
    fluid_ounce: f64,
}
fn print_selected_drink_details(drink_struct: Drink) {
    // println!("drink_struct.flavor {}", drink_struct.flavor); // why we are not using this? : // `Flavors` cannot be formatted with the default formatter // error[E0277]: `Flavors` doesn't implement `std::fmt::Display`

    // to print enum data:

    match drink_struct.flavor {
        Flavors::VirginMogito => println!("drink_struct.flavor: {}", "VirginMogito"),
        Flavors::Soda => println!("drink_struct.flavor: {}", "Soda"),
        Flavors::Fruity => println!("drink_struct.flavor: {}", "Fruity"),
    };
    println!("drink_struct.fluid_ounce {}", drink_struct.fluid_ounce);
}
fn main() {
    let fruity_struct = Drink {
        flavor: Flavors::Fruity,
        fluid_ounce: 6.2,
    };
    print_selected_drink_details(fruity_struct);

    let soda_struct = Drink {
        flavor: Flavors::Soda,
        fluid_ounce: 4.7,
    };
    print_selected_drink_details(soda_struct);
}
