// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id_number: i32,
}

fn main() {
    let struct_instance = Grocery {
        quantity: 10,
        id_number: 101,
    };

    print_grocery_quantity(&struct_instance);
    print_grocery_id(&struct_instance);
}

fn print_grocery_quantity(struct_instance: &Grocery){
    println!("{}", struct_instance.quantity);
}

fn print_grocery_id(struct_instance: &Grocery){
    println!("{}", struct_instance.id_number);
}