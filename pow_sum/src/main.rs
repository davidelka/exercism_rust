pub fn square_of_sum(n: u32) -> u32 {
    (((1 + n) * n) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n + 1 {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    let sum = sum_of_squares(n);
    let square_of_sum = square_of_sum(n);

    sum.abs_diff(square_of_sum)
}

fn main() {
    assert_eq!(25_164_150, difference(100));
}
