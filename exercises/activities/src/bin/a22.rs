// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b==0{
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]

mod check{
    use crate::*;
    #[test]
    fn test_divide(){
        let check = div(10, 2);
        let expected_result = Some(5);
        assert_eq!(check, expected_result, "divide functionality is wrong")
    }

    #[test]
    fn test_divideby0(){
        let check = div(10, 0);
        let expected_result = None;
        assert_eq!(check, expected_result, "divide functionality is wrong")
    }

    #[test]
    fn test_clamp_lower(){
        let check = clamp(10,100,1000);
        let expected_result = 100;
        assert_eq!(check, expected_result, "number should be equal to 100")
    }

    #[test]
    fn test_clamp_upper(){
        let check = clamp(4000,100,1000);
        let expected_result = 1000;
        assert_eq!(check, expected_result, "number should be equal to 1000")
    }

    #[test]
    fn test_clamp_at_limit(){
        let n_at_limit = 100;
        let check = clamp(n_at_limit,100,1000);
        let expected_result = n_at_limit;
        assert_eq!(check, expected_result, "number should be equal to n_at_limit")
    }

    #[test]
    fn test_concat(){
        let check = concat("a", "b");
        let expected_result = String::from("ab");
        assert_eq!(check, expected_result, "number should be equal to n_at_limit")
    }

}

