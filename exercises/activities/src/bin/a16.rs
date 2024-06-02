// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


// use std::ptr::null;

// #[derive(Debug)]
// struct Student <'a> {
struct Student {
    name: String,
    // locker_assignment: Option<& 'a str>
    locker_assignment: Option<i32> // since all students do not have a locker, hence we implemented options here
    // if every student was supposed to have a locker, then we would have simply used i32.
}
fn main() {
    let student_struct_instance = Student{
        name: "Rahul".to_owned(),
        locker_assignment: Some(5),
    };

    // println!("{:?}", student_struct_instance)
    match student_struct_instance.locker_assignment {
        Some(n)=>println!("name {:?} locker_assignment: {:?}", student_struct_instance.name, n),
        None => println!("name {:?} locker_assignment: Not Found", student_struct_instance.name),
    };
}
