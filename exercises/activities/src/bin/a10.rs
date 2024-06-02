// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let some_int:i32 = 1000;
    let expression_outcome = if some_int<=100{
       true
    }
    else{
       false
    };
    print_outcome_function(expression_outcome);
}


fn print_outcome_function(expression_outcome:bool){
    match expression_outcome{
        true=>println!("it's small"),
        false=>println!("it's big"),
    };
    
}
