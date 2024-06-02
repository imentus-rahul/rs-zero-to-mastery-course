// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person{
    name: String,
    age: i32,
    fav_color: String,
}

fn main() {
    let name1 = "tony".to_owned();
    let fav_color1 = "red".to_owned();

    let name2 = "elon".to_owned();
    let fav_color2 = "green".to_owned();

    let name3 = "jack sparrow".to_owned();
    let fav_color3 = "blue".to_owned();

    let person1 = Person{
        name: name1,
        age: 5,
        fav_color: fav_color1,
    };

    let person2 = Person{
        name: name2,
        age: 9,
        fav_color: fav_color2,
    };

    let person3 = Person{
        name: name3,
        age: 50,
        fav_color: fav_color3,
    };

    let vector_storing_person_structs = vec![person1,person2,person3];

    for person in vector_storing_person_structs{
        if person.age<=10{
            print_data(&person.name);
            print_data(&person.fav_color);
            // println!("{:?}, {:?}", person.name, person.fav_color);
        }
        else{

        };
    }
}

// Invoke using a ampersand: print_data(&person.name);
fn print_data(some_str: &str){
    println!("{:?}", some_str);
}