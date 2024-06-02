// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<T, U>
where
    T: Body, // generic over traits Body abd Color
    U: Color,
{
    body: T,
    color: U,
}

impl<T, U> Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    pub fn new(body: T, color: U) -> Self {
        Self { body, color }
    }
}

#[derive(Debug)]
struct Truck; // It is not necessary to have function implementations for vehicle body or color
#[derive(Debug)]
struct Car; // It is not necessary to have function implementations for vehicle body or color
#[derive(Debug)]
struct Red; // It is not necessary to have function implementations for vehicle body or color
#[derive(Debug)]
struct Blue; // It is not necessary to have function implementations for vehicle body or color

impl Body for Truck {}
impl Body for Car {}

impl Color for Red {}
impl Color for Blue {}

fn main() {
    let red_truck = Vehicle::new(Truck {}, Red {});
    println!(
        "🚀 ~ file: a30.rs ~ line 53 ~ fnmain ~ red_truck: {:?}",
        red_truck
    );
    let blue_car = Vehicle::new(Car {}, Blue {});
    println!(
        "🚀 ~ file: a30.rs ~ line 55 ~ fnmain ~ blue_car: {:?}",
        blue_car
    );
}
