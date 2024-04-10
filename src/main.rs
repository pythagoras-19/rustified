mod Array;


fn main() {
    let sentence = String::from("Hello my name is");
    let index = first_word(&sentence);
    println!("{}", index);
    let t = slices(&sentence);
    let mut arr = Array::Array::new(10);
    let size = arr.get_size();
    println!("{}", size);

    let e = 999;
    arr.set_element(0, e);
    arr.get_elements();
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