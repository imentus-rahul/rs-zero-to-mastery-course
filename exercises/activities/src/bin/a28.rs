// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
#[derive(Debug)]
struct Shoes(Color);
#[derive(Debug)]
struct Shirt(Color);
#[derive(Debug)]
struct Pants(Color);

// new types pattern
impl Shoes {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Green => Err("Green Shoes Not Allowed".to_owned()),
            // _ => Ok(Self(_)) //  ^ `_` not allowed here
            other => Ok(Self(other)),
        }
    }
}

// new types pattern
impl Shirt {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("Purple Shirt Not Allowed".to_owned()),
            // _ => Ok(Self(_)) //  ^ `_` not allowed here
            other => Ok(Self(other)),
        }
    }
}

// new types pattern
impl Pants {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Yellow => Err("Yellow Pants Not Allowed".to_owned()),
            // _ => Ok(Self(_)) //  ^ `_` not allowed here
            other => Ok(Self(other)),
        }
    }
}
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

fn print_shoes_color(color: Shoes) {
    println!("shoes color: {:?}", color);
}

fn print_shirt_color(color: Shirt) {
    println!("shirt color: {:?}", color);
}

fn print_pants_color(color: Pants) {
    println!("pants color: {:?}", color);
}

fn main() {
    // New Types
    let shoes_color = Shoes::new(Color::Black).unwrap(); // .unwrap only takes the OK value from a Result Returned
    let shirt_color = Shirt::new(Color::White).unwrap();
    let pants_color = Pants::new(Color::Black).unwrap();

    print_shoes_color(shoes_color);
    print_shirt_color(shirt_color);
    print_pants_color(pants_color);
}
