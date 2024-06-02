// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// Q) what if we replace str with &str

fn main() {
    let first_name = "Rahul";
    let last_name = "Gangwal";

    let received_data1: &str;
    let received_data2: &str;

    received_data1 = display_first_name(first_name);
    received_data2 = display_last_name(last_name);

    println!("{} {}", received_data1, received_data2);
}

fn display_first_name(first_name: &str) -> &str {
    println!("{:?}", first_name);
    return first_name;
}

fn display_last_name(last_name: &str) -> &str {
    println!("{:?}", last_name);
    return last_name;
}

// main1(); // The main function is the entrypoint function and hence doesn't require to be invoked seperately.
// to run the program, use the following command:
// cd exercises/activities
// cargo run --bin a1
