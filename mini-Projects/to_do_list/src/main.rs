// To Do List - add task, remove task, view task, exit
use std::io::{self, Write};
fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        // --- Menu Display ---
        println!("\n--- To-Do List Menu ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Task");
        println!("4. Edit Task");
        println!("5. Exit");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid Input");

        let choice: u32 = choice.trim().parse().expect("Invalid Number");

        match choice {
            1 => add_task(&mut task_list),
            2 => view_task(&task_list),
            3 => remove_task(&mut task_list),
            4 => edit_tast(&mut task_list),
            5 => {
                println!("Exiting ...");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
    print!("Please enter a description for the task: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid Input");

    let description = description.trim().to_string();

    if !description.is_empty() {
        task_list.push(description);
        println!("Success: Task Added");
    } else {
        println!("Error: Task Description can not be empty");
    }
}
fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are remove!");
        return;
    }
    view_task(task_list);

    print!("Please enter the task number to remove: ");
    io::stdout().flush().unwrap();

    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid Input");

    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > 0 && task_number <= task_list.len() {
                let removed_task = task_list.remove(task_number - 1);
                println!("Success: Removed task '{}'", removed_task);
            } else {
                println!("Error Invalid Task Number");
            }
        }
        Err(_) => {
            println!("Error: Please Enter a valid number");
        }
    }
}
fn view_task(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    } else {
        for (index, task) in task_list.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
    println!("-------------------------");
}

fn edit_tast(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are editable!");
        return;
    }
    view_task(task_list);

    print!("Please enter the task number to edit: ");
    io::stdout().flush().unwrap();

    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid Input");

    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > 0 && task_number <= task_list.len() {
                print!("Please Edit the task: {} ", task_number);
                
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Invalid Input");
                task_list.remove(task_number - 1);
                let description = description.trim().to_string();

                if !description.is_empty() {
                    task_list.push(description);
                    println!("Success: Task Edited");
                } else {
                    println!("Error: Task Description can not be empty");
                }
            } else {
                println!("Error Invalid Task Number");
            }
        }
        Err(_) => {
            println!("Error: Please Enter a valid number");
        }
    }
}
