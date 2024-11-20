mod task;

use std::io;
use std::collections::HashMap;

pub fn run_program() {
    let mut input = String::new();
    let mut tasks = HashMap::new();

    loop {
        input.clear();
        println!("\n\nMain menu:");
        println!("1. Add task");
        println!("2. Display tasks");
        println!("3. Mark a task as complete");
        println!("4. Delete task");
        println!("5. Exit\n");
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");
        let choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid input");
                continue;
            }
        };
        match choice {
            1 => task::add_task(&mut tasks),
            2 => task::display_tasks(&mut tasks),
            3 => task::complete_task(&mut tasks),
            4 => task::delete_task(&mut tasks),
            _ => break
        }
    }
}
