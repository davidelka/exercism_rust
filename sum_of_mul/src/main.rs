pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut v = vec![];

    for i in 1..=limit {
        if factors.iter().any(|&x| i % x == 0) {
            v.push(i);
        }
        else {
            v.push(0);
        }
    }
    v.iter().sum()

    // (1..=limit).filter(|i| factors.iter().any(|&x| x!= 0 && i % x == 0)).sum()
}
