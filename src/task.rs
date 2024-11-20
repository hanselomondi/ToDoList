use rand::Rng;
use std::io;

pub struct Task {
    id: u32,
    description: String,
    completed: bool
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            id: rand::thread_rng().gen_range(1, 1001),
            description: description,
            completed: false
        }
    }
}

pub fn add_task(task: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Enter the task description:");
    io::stdin().read_line(&mut description)
        .expect("Failed to read input");
    task.push(Task::new(description));
}

pub fn display_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No existing tasks!");
    } else {
        println!("Your tasks:");
        for task in tasks {
            let status = if task.completed { "✓" } else { "✗" };
            println!("{}: [{}] {}", task.id, status, task.description);
        }
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();
    println!("Enter the ID of the task you want to delete:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read the input");
    let task_id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    match tasks.iter().position(|task| task_id == task.id) {
        Some(index) => {
            tasks.remove(index);
            println!("Task successfully deleted");
        }
        None => {
            println!("No task with the ID {} exists", task_id);
        }
    }
}

pub fn complete_task(task_id: u32, tasks: &mut Vec<Task>) {}
