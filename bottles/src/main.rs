pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = vec![];

    for i in (start_bottles - take_down + 1..start_bottles + 1).rev() {
        song.push(create_verse(i));
    }
    song.join("\n\n")
}

fn create_verse(i: u32) -> String {
    let bottles_left = parse_num(i - 1).to_ascii_lowercase();
    let bottles = parse_num(i);
    let mut s = "s";
    if i == 1 {
        s = "";
    }
    let mut s_2 = "s";
    if i == 2 {
        s_2 = "";
    }

    format!(
        "{bottles} green bottle{s} hanging on the wall,\n{bottles} green bottle{s} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {bottles_left} green bottle{s_2} hanging on the wall."
    )
}

fn parse_num(num: u32) -> String {
    ONES[num as usize].to_string()
}

const ONES: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

fn main() {
    println!("dsfsd");
}

#[test]
fn first_generic_verse() {
    assert_eq!(
        recite(10, 10).trim(),
        concat!(
            "Ten green bottles hanging on the wall,\n",
            "Ten green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be nine green bottles hanging on the wall.\n",
            "\n",
            "Nine green bottles hanging on the wall,\n",
            "Nine green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be eight green bottles hanging on the wall.\n",
            "\n",
            "Eight green bottles hanging on the wall,\n",
            "Eight green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be seven green bottles hanging on the wall.\n",
            "\n",
            "Seven green bottles hanging on the wall,\n",
            "Seven green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be six green bottles hanging on the wall.\n",
            "\n",
            "Six green bottles hanging on the wall,\n",
            "Six green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be five green bottles hanging on the wall.\n",
            "\n",
            "Five green bottles hanging on the wall,\n",
            "Five green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be four green bottles hanging on the wall.\n",
            "\n",
            "Four green bottles hanging on the wall,\n",
            "Four green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be three green bottles hanging on the wall.\n",
            "\n",
            "Three green bottles hanging on the wall,\n",
            "Three green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be two green bottles hanging on the wall.\n",
            "\n",
            "Two green bottles hanging on the wall,\n",
            "Two green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be one green bottle hanging on the wall.\n",
            "\n",
            "One green bottle hanging on the wall,\n",
            "One green bottle hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be no green bottles hanging on the wall.",
        )
    );
}
