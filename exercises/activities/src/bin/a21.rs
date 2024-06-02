// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let new_user = User {
        user_id: 3,
        name: "elon".to_owned(),
    };

    let user_name = "sam";
    let option_result_i32_type = find_user(user_name); // apply the map combinators here
    // use option_result to directly initialize a variable in case of SOME value.


    // used closure and map combinator
    let option_result_user_type = find_user(user_name).map(|user_id| User{
        user_id: user_id,
        name: user_name.to_owned(),
    });

    match option_result_i32_type {
        Some(num) => define_struct(num, user_name),
        // Some(num)=>println!("{}",num),
        None => println!("user not found"),
    };
    println!("option_result_i32_type: {:?}",option_result_i32_type);

    // tried borrowing the reeference fixed my issue to print option in next 4 lines
    match &option_result_user_type {
        Some(user)=>println!("match some - &option_result_user_type - {:?}",user),
        None => println!("user not found"),
    };
    println!("option_result_user_type: {:?}",option_result_user_type);

}
 
fn define_struct(num:i32, user_name:&str){
    let some_struct = User {
        user_id: num,
        name: user_name.to_owned(),
    };
    
    println!("🚀 ~ file: a21.rs ~ line 47 ~ fndefine_struct ~ some_struct {:?}", some_struct);
}