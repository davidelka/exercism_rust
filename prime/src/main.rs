pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    (3..).step_by(2).filter(|x| is_prime(*x)).nth(n as usize - 1).unwrap()
}

fn is_prime(n: u32) -> bool {
    !(2..n.isqrt()+1).any(|d| n % d == 0)
}
#[test]
fn first_prime() {
    let output = nth(0);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
fn second_prime() {
    let output = nth(1);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
fn sixth_prime() {
    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);
}

#[test]
fn big_prime() {
    let output = nth(10_000);
    let expected = 104_743;
    assert_eq!(output, expected);
}

#[test]
fn very_big_prime() {
    let output = nth(2_000_000);
    println!("{output}")
}

fn main() {
    println!("Hello, world!");
}
