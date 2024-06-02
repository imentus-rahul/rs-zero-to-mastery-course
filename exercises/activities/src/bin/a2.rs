// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let num_1: i32 = 10;
    let num_2: i32 = 20;

    sum(num_1, num_2);
}

fn sum(num_1: i32, num_2: i32) -> i32 {
    let result = num_1 + num_2;
    display_results(result);
    return result;
}

fn display_results(result: i32) {
    println!("{:?} inside display_results", result);
}
