pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let num_len = num.to_string().len();

    for i in 1..num_len as u32 + 1 {
        let digit: u32 = num % 10_u32.pow(i) / (10_u32.pow(i - 1));
        sum += digit.pow(num_len as u32);
    }

    sum == num
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
fn single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
fn there_are_no_two_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
fn three_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
fn three_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
fn four_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_474))
}

#[test]
fn four_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_475))
}

#[test]
fn seven_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
fn seven_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_926_314))
}
