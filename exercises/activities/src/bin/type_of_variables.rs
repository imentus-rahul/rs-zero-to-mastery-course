// Rust program to print the type of variables

fn TypeOf<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s = "Hello";
    let n1 = 42;
    let n2 = 42.68;
    let b  = true;
    
    TypeOf(&s); 
    TypeOf(&n1);
    TypeOf(&n2);
    TypeOf(&b);
}