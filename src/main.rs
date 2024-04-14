mod linked_list;
mod array;
mod currencies;
mod my_networking;
mod guess;

use crate::currencies::Coin;
use crate::currencies::Dollar;

use colored::*;


fn main() {
    // array_ops();
    // linked_list_ops()
    // enum_ops();
    networking_ops();
}

fn networking_ops() {
    my_networking::start_server_and_client_threads();
}

fn enum_ops() {
    currencies::value_in_cents(Coin::Penny);
    currencies::value_in_dollars(Dollar::BenjaminFranklin);
    currencies::parse_thru_currencies();

}

fn linked_list_ops() {
    let node = linked_list::Node::new(65);
    let mut ll = linked_list::LinkedList::new(node);
    ll.append(66);
    ll.print();
}

fn array_ops() {
    let sentence = String::from("Hello my name is");
    let index = first_word(&sentence);
    println!("{}", index);
    let t = slices(&sentence);
    println!("{}", t);
    let mut arr = array::Array::new(10);
    let size = arr.get_size();
    println!("{}", size);
    println!("==RANDOMIZING INPUTS==");
    arr.randomize_inputs();
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
    let min = arr.get_min();
    println!("{:?}", min);
    let average = arr.get_average();
    println!("Average: {:?} ", average);
    let l = arr.to_linked_list();
    l.print();
    println!("array is empty: {}", arr.is_empty());
    arr.reverse();
    arr.get_elements();
    arr.remove_at(3);
    arr.get_elements();
    arr.randomize_inputs();
    println!("Sorted: {}", arr.is_sorted());
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