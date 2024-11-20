use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Task {
    id: u32,
    description: String,
    completed: bool
}

impl Task {
    fn new(description: String, tasks: &HashMap<u32, Task>) -> Task {
        Task {
            id: (tasks.len() + 1).try_into().unwrap(),
            description: description,
            completed: false
        }
    }
}

pub fn add_task(tasks: &mut HashMap<u32, Task>) {
    let mut description = String::new();
    println!("Enter the task description:");
    io::stdin().read_line(&mut description)
        .expect("Failed to read input");
    tasks.insert(
        (tasks.len() + 1).try_into().unwrap(),
        Task::new(
            description.trim().to_string(),
            tasks
        )
    );
    println!("Task created successfully!");
}

pub fn display_tasks(tasks: &HashMap<u32, Task>) {
    if tasks.is_empty() {
        println!("No existing tasks!");
    } else {
        println!("Your tasks:");
        for task in tasks.values() {
            let status = if task.completed { "✓" } else { "✗" };
            println!("{}: [{}] {}", task.id, status, task.description);
        }
    }
}

pub fn delete_task(tasks: &mut HashMap<u32, Task>) {
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
    if tasks.contains_key(&task_id) {
        println!("Task {:#?} deleted", tasks.remove(&task_id));
    } else {
        println!("No task with the ID {} exists", task_id);
    }
}

pub fn complete_task(tasks: &mut HashMap<u32, Task>) {
    let mut input = String::new();
    println!("Enter ID of task to mark as complete:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let task_id = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    if tasks.contains_key(&task_id) {
        if let Some(task) = tasks.get_mut(&task_id) {
            task.completed = true;
            println!("Task marked as completed");
        }
    } else {
        println!("No task with the ID {} exists", task_id);
    }
}
