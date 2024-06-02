use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let random_number: u8 = random();
    println!(
        "🚀 ~ file: random_numbers.rs ~ line 4 ~ fnmain ~ random_number {}",
        random_number
    ); // any random no in u8 range // picking values from collection

    let mut random_number_generator = thread_rng();
    let random_number = random_number_generator.gen_range(0..=10);
    println!(
        "🚀 ~ file: random_numbers.rs ~ line 8 ~ fnmain ~ random_number {}",
        random_number
    ); // any random no in given range // picking values from collection

    let some_vec: Vec<u8> = vec![10, 30, 50];
    let random_number = some_vec.choose(&mut random_number_generator);
    println!(
        "🚀 ~ file: random_numbers.rs ~ line 13 ~ fnmain ~ random_number {:?}",
        random_number
    ); // random choose method from collection values // picking values from collection

    // changing to mutable state
    let mut some_vec = some_vec;
    println!("🚀 ~ file: random_numbers.rs ~ line 24 ~ fnmain ~ UNSHUFFLED some_vec {:?}", some_vec); 
    // shuffling of some_vec values in vector itself
    some_vec.shuffle(&mut random_number_generator);
    println!("🚀 ~ file: random_numbers.rs ~ line 24 ~ fnmain ~ SHUFFLED some_vec {:?}", some_vec);

    // SEEDING - NOT CRYPTOGRAPHICALLY SECURED
    // RAND_PCG: PCG, A Family of Better Random Number Generators
    // A permuted congruential generator (PCG) is a pseudorandom number generation algorithm 
    // PCG-64 is a 128-bit implementation of O'Neill's permutation congruential generator PCG
    // Unlike many general-purpose RNGs, they are also hard to predict.
    // yet many Random Number Generators RNGs fail statistical tests for randomness. - Not Actually Random, Predictable & Insecure
    // https://github.com/rust-random/book/blob/master/src/guide-seeding.md

    let mut better_random_no_generator_pcg = Pcg64::seed_from_u64(10); // seed from u64
    println!("{:?}", better_random_no_generator_pcg.gen::<u8>());

    let mut better_random_no_generator_pcg:Pcg64 = Seeder::from("some seed value").make_rng(); // seed from string
    println!("{:?}", better_random_no_generator_pcg.gen::<u8>());

    // Distributions Random number generators
    // random number in a range
    let some_range = Uniform::from(5..500);
    println!("🚀 ~ file: random_numbers.rs ~ line 49 ~ fnmain ~ some_range {:?}", some_range);
    let random_number = some_range.sample(&mut random_number_generator);
    println!("🚀 ~ file: random_numbers.rs ~ line 51 ~ fnmain ~ random_number {:?}", random_number);

    // Next: VRFs




}
