// Topic: Generics & Functions
//
// Requirements:
// * Create a function that accepts the Priority trait as a generic parameter
//   * The function should print out the guest and their priority
// * Use the function with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print out the information
// * Use the compiler to guide you to the correct generic constraints needed

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// + std::fmt::Debug
fn generic_function_invoking_trait<T: Priority + std::fmt::Debug>(guest_type: T) {
    println!("🚀 ~ file: a29.rs ~ line 39 ~ guest_type: {:?}", guest_type);
    let result = guest_type.get_priority();
    println!("🚀 ~ file: a29.rs ~ line 40 ~ result: {:?}", result);
}
fn main() {
    let guest1 = Guest {};
    let guest2 = ImportantGuest {};
    let guest3 = Guest {};
    generic_function_invoking_trait(guest1);
    generic_function_invoking_trait(guest2);
    generic_function_invoking_trait(guest3);
}
