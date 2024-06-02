// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
#[derive(Debug)]
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColour,
}

#[derive(Debug)]
struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

#[derive(Debug)]
enum BoxColour {
    Red,
    Green,
    Blue,
}

fn main() {
    ShippingBox::create_shipping_box_instance()
}

impl ShippingBox {
    fn create_shipping_box_instance() {
        let dimensions_instance = Dimensions{
            height: 36.0,
            width: 24.0,
            depth: 12.0
        };

        let shipping_box_instance = ShippingBox {
            dimensions: dimensions_instance,
            weight: 48.0,
            color: BoxColour::Red,
        };

        // trivial method to invoke a method inside impl
        ShippingBox::print_shipping_box_instance_details(shipping_box_instance)

        // below both ways won't work untill the function has a self parameter
        // shipping_box_instance.print_shipping_box_instance_details(shipping_box_instance) // found the following associated functions; to be used as methods, functions must have a `self` parameter
        // shipping_box_instance.print_shipping_box_instance_details() // found the following associated functions; to be used as methods, functions must have a `self` parameter

        // using self function
        // shipping_box_instance.print_shipping_box_instance_details_with_self(shipping_box_instance) // supplied 1 argument, expected 0 arguments
        // shipping_box_instance.print_shipping_box_instance_details_with_self() // this will work
    }

    fn print_shipping_box_instance_details(shipping_box_instance: ShippingBox) {
        // println!("{}", shipping_box_instance.dimensions); // `Dimensions` cannot be formatted with the default formatter
        
        // printing stuct using #[derive(Debug)]
        println!("{:?}", shipping_box_instance.dimensions);

        // printing whole struct consisting another struct and enum
        println!("{:?}", shipping_box_instance);

        // other way to print red color
        match shipping_box_instance.color {
            BoxColour::Red => println!("Red"),
            _ => println!("No-use"),
        };
    }

    fn print_shipping_box_instance_details_with_self(&self) {
        // println!("{}", self.dimensions); // `Dimensions` cannot be formatted with the default formatter
        
        // printing stuct using #[derive(Debug)]
        println!("{:?}", self.dimensions);

        // printing whole struct consisting another struct and enum
        println!("{:?}", self);

        // other way to print red color
        match self.color {
            BoxColour::Red => println!("Red"),
            _ => println!("No-use"),
        };
    }
}
