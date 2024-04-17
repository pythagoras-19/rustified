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
mod options;


fn main() {
    // array_ops();
    // linked_list_ops();
    // enum_ops();
    // networking_ops();
    // thread_ops();
    // channel_ops();
    // path_ops();
    // file_ops();
    // signal_ops();
    option_ops();
}

fn signal_ops() { my_signals::entry(); }

fn file_ops() { files::entry(); }

fn path_ops() { paths::entry(); }

fn channel_ops() { channels::channels(); }

fn thread_ops() { thread::entry(); }

fn networking_ops() { my_networking::entry(); }

fn enum_ops() { currencies::entry(); }

fn linked_list_ops() { linked_list::entry(); }

fn array_ops() { array::entry(); }

fn option_ops() { options::entry(); }