mod manage_tasks;
mod manage_cli;

use crate::manage_tasks::tasks;
use crate::manage_cli::cli;

fn main() {
    let mut vec_of_tasks: Vec<tasks::Task> = Vec::new();
    let mut user_input: String = String::from("");

    loop {
        println!("Enter a task name: ");
        user_input = cli::get_input();

        let user_input = user_input.trim().to_lowercase();

        if user_input == "exit" {
            println!("Exiting...");
            break;
        }

        let new_task = tasks::Task::new(user_input);
        vec_of_tasks.push(new_task);
    }

    // before completion
    for task in &vec_of_tasks {
        println!("{}: {}", task.name, task.completed);
    }

    for task in &mut vec_of_tasks {
        task.mark_complete();
    }

    // after completion
    println!();
    for task in &vec_of_tasks {
        println!("{}: {}", task.name, task.completed);
    }
}