// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


struct Customer{
    age: i32,
}

fn age_gate(age: i32)->Result<String,String>{
    if age>21{
        return Ok("Eligible to purchase".to_owned())
    }
    else{
        return Err("Not Eligible to purchase, because of age limit".to_owned())
    }

}
fn main() {
    let customer_struct_instance = Customer{
        age: 12
    };
    let return_result = age_gate(customer_struct_instance.age);

    println!("{:?}",return_result);

    // printing result with match
    match return_result{
        Result::Ok(data)=>println!("{:?}", data),
        Result::Err(data)=>println!("{:?}", data),
    };
}
