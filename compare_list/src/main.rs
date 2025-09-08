#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    // Check if A is a sublist of B
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    // Check if A is a superlist of B (B is a sublist of A)
    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    // Lists are unequal
    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    // Empty list is a sublist of any list
    if needle.is_empty() {
        return true;
    }

    // If needle is longer than haystack, it can't be a sublist
    if needle.len() > haystack.len() {
        return false;
    }

    // Check all possible starting positions in haystack
    for i in 0..=(haystack.len() - needle.len()) {
        if haystack[i..i + needle.len()] == *needle {
            return true;
        }
    }

    false
}

fn main() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[2, 3, 4];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}
