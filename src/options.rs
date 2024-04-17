pub fn entry() {
    print_option(Option::None);
}

pub fn print_option(opt: Option<i64>) {
    match opt {
        Some(value) => println!("{}", value),
        None => println!("None Value!"),
    };
}