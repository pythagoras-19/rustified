mod linked_list;
mod array;
mod currencies;
mod my_networking;
mod guess;
mod chat_server;
mod thread;
mod channels;
mod paths;
mod files;
mod my_signals;

use crate::currencies::Coin;
use crate::currencies::Dollar;
use crate::paths::create_valid_path;

fn main() {
    // array_ops();
    linked_list_ops();
    // enum_ops();
    // networking_ops();
    // thread_ops();
    // channel_ops();
    // path_ops();
    // file_ops();
    // signal_ops();
}

fn signal_ops() { my_signals::entry(); }

fn file_ops() { files::entry(); }

fn path_ops() { paths::create_valid_path(); }

fn channel_ops() { channels::channels(); }

fn thread_ops() { thread::entry(); }

fn networking_ops() { my_networking::entry(); }

fn enum_ops() { currencies::entry(); }

fn linked_list_ops() { linked_list::entry(); }

fn array_ops() { array::entry(); }

fn print_option(opt: Option<i64>) {
    match opt {
        Some(value) => println!("{}", value),
        None => println!("None Value!"),
    };
}
