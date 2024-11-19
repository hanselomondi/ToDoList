use rand::Rng;

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

pub fn add_task(task: &mut Vec<Task>) {}

pub fn view_tasks(tasks: &Vec<Task>) {}

pub fn delete_task(task_id: u32, tasks: &mut Vec<Task>) {}

pub fn complete_task(task_id: u32, tasks: &mut Vec<Task>) {}
