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

enum Colours {
    Red,
    Green,
    Blue,
}
fn main() {
    enum_invoker_function(Colours::Red)
    
}

fn enum_invoker_function(enum_invoker:Colours)
{
    // let enum_invoker = Colours::Red;
    // println!("🚀 ~ file: a7.rs ~ line 20 ~ fnmain ~ enum_invoker {}", enum_invoker);  
    match enum_invoker{
        Colours::Red=>println!("{}","Red"),
        Colours::Green=>println!("{}","Green"),
        Colours::Blue=>println!("{}","Blue"),
    };
}