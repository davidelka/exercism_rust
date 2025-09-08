pub fn square(s: u32) -> u64 {
    2.pow(s -1)
}

pub fn total() -> u64 {
    let sum:u64 = 0;
    for i in 1..65{
        sum += square(i);
    }
    sum
}

pub fn total_map() -> u64 {
    (1..65).map(square).sum()
}


fn main() {
    println!("hello world");
}