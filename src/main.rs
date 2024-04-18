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
mod pipes;
mod my_os_1;

use std::io;
use std::io::*;
use colored::*;


fn main() {
    print_title();
    main_menu();
}

fn print_title() {
    println!("    ____  __  _____________________________________ ");
    println!("   / __ \\/ / / / ___/_  __/  _/ ____/  _/ ____/ __ \\");
    println!("  / /_/ / / / /\\__ \\ / /  / // /_   / // __/ / / / /");
    println!(" / _, _/ /_/ /___/ // / _/ // __/ _/ // /___/ /_/ / ");
    println!("/_/ |_|\\____//____//_/ /___/_/   /___/_____/_____/  ");
}

fn main_menu() {
    loop {
        println!("============Main Menu=============");
        println!("1. Array Operations");
        println!("2. Linked List Operations");
        println!("3. Enum Operations");
        println!("4. Networking Operations");
        println!("5. Thread Operations");
        println!("6. Channel Operations");
        println!("7. Path Operations");
        println!("8. File Operations");
        println!("9. Signal Operations");
        println!("10. Option Operations");
        println!("11. Pipe Operations");
        println!("12. OS (1) Operations");
        println!("{}", "0. Exit".red());
        println!("===================================");

        // Prompt the user for input
        print!("{}", "Enter your choice: ".green());
        io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and parse input as integer
        let choice = input.trim().parse::<u32>();

        // Handle invalid input
        let choice = match choice {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input! Please enter a number.".red());
                continue;
            }
        };

        match choice {
            1 => array_ops(),
            2 => linked_list_ops(),
            3 => enum_ops(),
            4 => networking_ops(),
            5 => thread_ops(),
            6 => channel_ops(),
            7 => path_ops(),
            8 => file_ops(),
            9 => signal_ops(),
            10 => option_ops(),
            11 => pipes_ops(),
            12 => os_1_ops(),
            0 => {
                println!("Exiting...");
                print_title();
                break;
            }
            _ => println!("{}", "Invalid choice! Please enter a number between 0 and 10.".red()),
        }
    }
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

fn pipes_ops() { pipes::entry(); }

fn os_1_ops() { my_os_1::entry(); }