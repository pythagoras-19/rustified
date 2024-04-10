mod Array;

fn main() {
    println!("Hello, world!");
    let sentence = String::from("Hello my name is");
    let index = first_word(&sentence);
    println!("{}", index);
    let t = slices(&sentence);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'H' {
            return i;
        }
    }

    s.len()
}

fn slices(s: &String) -> bool {
    let h = &s[0..5];
    println!("{}", &h);
    true
}