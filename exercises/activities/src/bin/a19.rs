// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
fn main() {
    let mut no_of_items = HashMap::new();
    no_of_items.insert("Chairs", 5);
    no_of_items.insert("Beds", 3);
    no_of_items.insert("Tables", 2);
    no_of_items.insert("Couches", 0);

    let mut total_no_of_items = 0;

    for (name,qty) in no_of_items.iter(){
        println!("{:?}", name);
        if qty>&0{
            println!("{:?}", qty);
        }
        else{
            println!("{:?}", "out of stock");
        }
        total_no_of_items = total_no_of_items + qty;
    }
    
    println!("{:?}", total_no_of_items)

}
