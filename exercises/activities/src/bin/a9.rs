// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn returns_tuple() -> (f64, f64) {
    let some_tuple = (12.5, 10.6);
    return some_tuple;
}
fn main() {
    let (x_cord, y_cord) = returns_tuple();

    // match y_cord{
    //     (y_cord<5)=>println!("less than 5"),
    //     (y_cord>5)=>println!("greater than 5"),
    //     (y_cord==5)=>println!("equal to 5"),
    // };

    if y_cord < 5.0 {
        println!("less than 5");
    } else if y_cord > 5.0 {
        println!("greater than 5");
    } else {
        println!("equal to 5");
    }
}
