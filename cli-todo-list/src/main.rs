mod manage_tasks;
mod manage_cli;

use crate::manage_tasks::tasks;
use crate::manage_cli::cli;

fn main() {
    let mut task1 = tasks::Task::new(String::from("Finish Rust project"));
    println!("Task: {}, Completed: {}", task1.name, task1.completed);
    
    task1.mark_complete();
    println!("Task: {}, Completed: {}", task1.name, task1.completed);

    println!("Please enter a new task:");
    let new_task_name = cli::get_input();
    let mut task2 = tasks::Task::new(new_task_name);
    println!("New Task: {}, Completed: {}", task2.name, task2.completed);

    task2.mark_complete();
    println!("Task: {}, Completed: {}", task2.name, task2.completed);
}