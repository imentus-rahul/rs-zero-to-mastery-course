#[derive(Debug)]
enum RandomEnum {
    SomeString(String),
    // SomeFunction(fn ()),
}
 

// fn some_function() {
//     println!("{}", 5)
// }

fn main() {
    enum_invoker_function(RandomEnum::SomeString("RAHUL".to_owned()));
    // enum_invoker_function(RandomEnum::SomeFunction(some_function()));
    
}

fn enum_invoker_function(enum_invoker:RandomEnum)
{
    match enum_invoker{
        RandomEnum::SomeString(x)=>println!("{}",x),
        // RandomEnum::SomeFunction(y)=>println!("{}",y),
    };
}

// enum Color {
//     Red,
//     Green,
//     Blue
// }

// impl Color {
//     fn print(&self) {
//         match self {
//             Color::Red => println!("Red color"),
//             Color::Green => println!("Green color"),
//             Color::Blue => println!("Blue color"),
//         }
//     }
// }

// fn main() {
//     let color = Color::Green;
//     color.print();
// }