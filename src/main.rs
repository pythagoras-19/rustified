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

mod graphics {
    pub mod ballin;
    pub mod spinning_square;
    pub mod ellipse;
    pub mod starry_night;
    pub mod snake;
}

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
    let menu_items = vec![
        "1. Array Operations", "2. Linked List Operations", "3. Enum Operations",
        "4. Networking Operations", "5. Thread Operations", "6. Channel Operations",
        "7. Path Operations", "8. File Operations", "9. Signal Operations",
        "10. Option Operations", "11. Pipe Operations", "12. OS (1) Operations",
        "13. Init Chat Server", "14. Ballin Ops (Graphics)", "15. Spinning Square (Graphics)",
        "16. Ellipse (Graphics)", "17. Starry Night (Graphics)", "18. Snake Game (Graphics)"
    ];
    let number_of_options = menu_items.len();
    loop {
        let col_width = 60;
        println!("============Main Menu=============");
        for i in (0..menu_items.len()).step_by(3) {
            let first = menu_items.get(i).unwrap_or(&"");
            let second = menu_items.get(i + 1).unwrap_or(&"");
            let third = menu_items.get(i + 2).unwrap_or(&"");
            println!("{:width$}{:width$}{:width$}", first, second, third, width = col_width);
        }
        println!("{}", "0. Exit".red());
        println!("===================================");

        // Prompt the user for input
        print!("{}", "Enter your choice: ".green());
        stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

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
            13 => chat_server_ops(),
            14 => graphics_ballin_ops(),
            15 => spinning_square(),
            16 => ellipse(),
            17 => starry_night(),
            18 => snake_game(),
            0 => {
                println!("Exiting...");
                print_title();
                break;
            }
            _ => println!("{}", format!("Invalid choice! Please enter a number between 0 and {}.", number_of_options).red()),
        }
    }
}

fn signal_ops() { my_signals::entry(); }

fn file_ops() { files::entry(); }

fn path_ops() { paths::entry(); }

fn channel_ops() { channels::entry(); }

fn thread_ops() { thread::entry(); }

fn networking_ops() { my_networking::entry(); }

fn enum_ops() { currencies::entry(); }

fn linked_list_ops() { linked_list::entry(); }

fn array_ops() { array::entry(); }

fn option_ops() { options::entry(); }

fn pipes_ops() { pipes::entry(); }

fn os_1_ops() { my_os_1::entry(); }

fn chat_server_ops() { chat_server::entry(); }

fn graphics_ballin_ops() { graphics::ballin::entry(); }

fn spinning_square() { graphics::spinning_square::entry(); }

fn ellipse() { graphics::ellipse::entry(); }

fn starry_night() { graphics::starry_night::entry(); }

fn snake_game() {
    println!("{}", "DEPRECATED FOR NOW.".red())
    //graphics::snake::entry();
}