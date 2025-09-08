use std::ops::Index;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for (i, line) in garden.iter().enumerate() {
        let mut v_line = Vec::new();
        for j in 0..line.len() {
            if line.as_bytes().index(j) == "*".as_bytes().index(0) {
                v_line.push("*".to_string());
            } else {
                v_line.push(add_flower(garden, i, garden.len(), j, line.len()));
            }
        }
        v.push(v_line.iter().map(|n| n.to_string()).collect::<String>());
    }
    v
}

fn add_flower(garden: &[&str], i: usize, i_len: usize, j: usize, j_len: usize) -> String {
    let mut count = 0;
    let mut v = Vec::new();

    let i = i as i32;
    let j = j as i32;
    let j_len = j_len as i32;
    let i_len = i_len as i32;

    v.push((i - 1, j - 1));
    v.push((i - 1, j));
    v.push((i - 1, j + 1));

    v.push((i, j - 1));
    v.push((i, j + 1));

    v.push((i + 1, j - 1));
    v.push((i + 1, j));
    v.push((i + 1, j + 1));

    v.retain(|(x, y)| x >= &0 && y >= &0 && x < &i_len && y < &j_len);
    let v: Vec<(usize, usize)> = v
        .into_iter()
        .map(|(x, y)| (x as usize, y as usize))
        .collect();

    for (x, y) in v {
        if garden[x].as_bytes()[y] == "*".as_bytes()[0] {
            count += 1;
        }
    }
    if count == 0 {
        return " ".to_string();
    }
    count.to_string()
}

fn main() {
    let (input, expected) = (&["   ", " * ", "   "], &["111", "1*1", "111"]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
