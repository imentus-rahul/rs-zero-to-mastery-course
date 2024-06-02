// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let some_vector = vec![10,20,30,40];
    for num in &some_vector{
        // *num ==> value at borrowed reference
        if *num == 30{ // cannot compare a borrowed integer with regular integer ==>`can't compare `&{integer}` with `{integer}`
            println!("Thirty");
        }
        else{
            println!("{}", num);
        };
    }

    // for num in &some_vector{
    //    match num{ // works with both num + *num
    //     30=>println!("thirty"),
    //     _=>println!("{}", num),
    //    }
    // }

    println!("number of elements: {}", some_vector.len());

}
