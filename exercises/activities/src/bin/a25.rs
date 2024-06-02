// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calc_perimeter(&self)->i32; // will require &self below in impl block as well
}
struct Square {
    side: i32,
}
struct Triangle {
    side1: i32,
    side2: i32,
    side3: i32,
}

impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        // let perimeter_result = 
        // perimeter_result     
        self.side * 4
    }
}

impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        // let perimeter_result = 
        // perimeter_result
        self.side1 + self.side2 + self.side3
    }
}

// generic function
fn instance_for_invoking_trait(perimeter_param: impl Perimeter) {
    let result = perimeter_param.calc_perimeter();
    println!(
        "🚀 ~ file: a25.rs ~ line 43 ~ fninstance_for_invoking_trait ~ result {:?}",
        result
    );
}
fn main() {
    instance_for_invoking_trait(Square { side: 4 });

    instance_for_invoking_trait(Triangle {
        side1: 4,
        side2: 10,
        side3: 5,
    });
}
