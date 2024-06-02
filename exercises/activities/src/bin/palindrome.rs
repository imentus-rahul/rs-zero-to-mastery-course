fn palindrome() {
    let some_str = "dharmendra";
    let str_len: usize = some_str.len();
    println!("{}", str_len);
    let mut j: usize = str_len-1;
    let mut palindrome_status: bool = true;

    for (index, ch) in some_str.chars().enumerate() {
        println!("{} {}", ch, index);
        if (index < str_len / 2) {
            // some_str.as_str().chars().nth(index).unwrap();
            // if (some_str[index] == some_str[j]){
            if (some_str.chars().nth(index).unwrap() == some_str.chars().nth(j).unwrap()) { // since we are using usize
                println!(
                    "First half {}, and Second half {} are equal",
                    some_str.chars().nth(index).unwrap(),
                    some_str.chars().nth(j).unwrap()
                )
            } else {
                palindrome_status = false;
            }
            j = j - 1;
        }
    }
    println!("Palindrome Status: {}", palindrome_status);
}

fn main() {
    palindrome();
}
