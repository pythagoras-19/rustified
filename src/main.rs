mod array;


fn main() {
    let sentence = String::from("Hello my name is");
    let index = first_word(&sentence);
    println!("{}", index);
    let t = slices(&sentence);
    println!("{}", t);
    let mut arr = array::Array::new(10);
    let size = arr.get_size();
    println!("{}", size);

    let e = 999;
    arr.set_element(0, e);
    let mut it = 0;
    while it < arr.get_size() {
        if it == 3 {
            arr.set_element(it, 1000);
        } else {
            arr.set_element(it,  300);
        }
        it += 1;
    }
    arr.get_elements();
    println!();
    arr.get_element(3);
    let m = arr.get_max();
    print_option(m);
    let most_occur = arr.get_most_occurring();
    print_option(most_occur);
    let median = arr.get_median();
    print_option2(median);
    arr.get_location();
    arr.pointer_stuff();
    let sum = arr.get_sum();
    println!("{}", &sum);
}

fn print_option(opt: Option<i64>) {
    match opt {
        Some(value) => println!("{}", value),
        None => println!("None Value!"),
    };
}

fn print_option2(opt: Option<f64>) {
    match opt {
        Some(value) => println!("{}", value),
        None => println!("None value!"),
    };
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