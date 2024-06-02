// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
}

fn print_customize_message(choice: PowerStates) {
    match choice {
        PowerStates::Off => println!("Initiated Off"),
        PowerStates::Sleep => println!("Initiated Sleep"),
        PowerStates::Reboot => println!("Initiated Reboot"),
        PowerStates::Shutdown => println!("Initiated Shutdown"),
        _ => println!("Initiated Shutdown"),
    };
}
use std::io;
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn main() {
    // let mut user_opt: String = "sHuTdOwN".to_owned();
    let res: io::Result<String> = get_input();
    // let mut user_opt: String = res()

    // let mut user_opt_lowercase: String = user_opt.to_lowercase();
    // let normalize_input:String = user_opt_lowercase.remove(0).to_uppercase().to_string() + &user_opt_lowercase;
    // println!(
    //     "🚀 ~ file: a20cle.rs ~ line 46 ~ fnmain ~ normalize_input {:?}",
    //     normalize_input
    // );

    let string_off: String = String::from("Off");
    let string_sleep: String = String::from("Sleep");
    let string_reboot: String = String::from("Reboot");
    let string_shutdown: String = String::from("Shutdown");

    let str_off = "Off";
    let str_sleep = "Sleep";
    let str_reboot = "Reboot";
    let str_shutdown = "Shutdown";

    // // match using string
    // match normalize_input {
    //     string_off=>print_customize_message(PowerStates::Off),
    //     string_sleep=>print_customize_message(PowerStates::Sleep),
    //     string_reboot=>print_customize_message(PowerStates::Reboot),
    //     string_shutdown=>print_customize_message(PowerStates::Shutdown),
    //     _=>println!("normalize_input mismatch")
    // };

    // // match using enum
    match res {
        Ok(some_str) => {
            println!("{}", some_str);
            let mut user_opt_lowercase: String = some_str.to_lowercase();
            let normalize_input: String =
                user_opt_lowercase.remove(0).to_uppercase().to_string() + &user_opt_lowercase;
            println!(
                "🚀 ~ file: a20cle.rs ~ line 46 ~ fnmain ~ normalize_input {:?}",
                normalize_input
            );
            

            // // match using string
            // match normalize_input {
            //     // "Shutdown" => print_customize_message(PowerStates::Shutdown),
            //     string_off => print_customize_message(PowerStates::Off),
            //     string_sleep => print_customize_message(PowerStates::Sleep),
            //     string_reboot => print_customize_message(PowerStates::Reboot),
            //     string_shutdown => print_customize_message(PowerStates::Shutdown),
            //     _ => println!("normalize_input mismatch"),
            // };

             // match using &str
             match normalize_input.as_str() {
                // "Shutdown" => print_customize_message(PowerStates::Shutdown),
                str_off => print_customize_message(PowerStates::Off),
                str_sleep => print_customize_message(PowerStates::Sleep),
                str_reboot => print_customize_message(PowerStates::Reboot),
                str_shutdown => print_customize_message(PowerStates::Shutdown),
                _ => println!("normalize_input mismatch"),
            };
        }
        Err(_) => println!("do nothing"),
    };
}
