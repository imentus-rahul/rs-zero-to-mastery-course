// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account

use std::iter::Sum;

trait SumForALLMaterial {
    fn amount(&self) -> f64;
}
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

struct Carpet(f64);
impl SumForALLMaterial for Carpet {
    fn amount(&self) -> f64 {
        // self is the carpet area
        self.0 * 10.0
    }
}
struct Tile(f64);
impl SumForALLMaterial for Tile {
    fn amount(&self) -> f64 {
        // self is the carpet area
        self.0 * 15.0
    }
}
struct Wood(f64);
impl SumForALLMaterial for Wood {
    fn amount(&self) -> f64 {
        // self is the carpet area
        self.0 * 20.0
    }
}

fn main() {
    let carper_square_meter: f64 = 20.0;
    let tile_square_meter: f64 = 10.0;
    let wood_square_meter: f64 = 30.0;

    // trait objects
    let carpet_total_per_category = Box::new(Carpet(carper_square_meter));
    let tile_total_per_category = Box::new(Tile(tile_square_meter));
    let wood_total_per_category = Box::new(Wood(wood_square_meter));

    let total_vec: Vec<Box<dyn SumForALLMaterial>> = vec![carpet_total_per_category, tile_total_per_category, wood_total_per_category];
    let total_sum = calc_total(&total_vec);
    println!("🚀 ~ file: a31.rs ~ line 62 ~ fnmain ~ total_sum: {:?}", total_sum);
}

fn calc_total(sales: &Vec<Box<dyn SumForALLMaterial>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}
