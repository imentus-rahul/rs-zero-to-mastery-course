// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

use std::vec;

#[derive(Debug)]
enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}
fn main() {
    let vec_containing_tickets_enum = vec![
        Tickets::Backstage(50.0, "tony".to_owned()),
        Tickets::Vip(30.0, "elon".to_owned()),
        Tickets::Standard(15.0),
    ];
    for each_vec in vec_containing_tickets_enum{
        match each_vec {
            Tickets::Backstage(price, holder) => println!("price: {:?}, holder: {:?}", price, holder),
            Tickets::Vip(price, holder) => println!("price: {:?}, holder: {:?}", price, holder),
            Tickets::Standard(price) => println!("price: {:?}", price),
            // _ => println!("anything"),
        };
    }
   
}
