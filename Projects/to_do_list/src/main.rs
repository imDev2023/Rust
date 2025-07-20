// To Do List - add task, remove task, view task, exit
use std::io::{self, Write};
#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: u8) -> Self {
        Self {
            description: description,
            priority: priority,
            completed: false,
        }
    }
}

struct TaskList {
    task_list: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        Self {
            task_list: Vec::new(),
        }
    }
    fn add_task(&mut self) {
        print!("Please enter a description for the task: ");
        io::stdout().flush().unwrap();

        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Invalid Input");

        let description = description.trim().to_string();

        print!("Please enter a priority for the task 1 to 5 :  ");
        io::stdout().flush().unwrap();

        let mut priority = String::new();
        io::stdin().read_line(&mut priority).expect("Invalid Input");

        let priority = priority.trim().parse::<u8>().expect("Invalid Input");

        if !description.is_empty() && (1..=5).contains(&priority) {
            self.task_list.push(Task::new(description, priority));
            println!("Success: Task Added");
        } else {
            println!("Error: Task Description pr priority is not correct");
        }
    }
    fn remove_task(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks are remove!");
            return;
        }
        self.view_task();

        print!("Please enter the task number to remove: ");
        io::stdout().flush().unwrap();

        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid Input");

        match task_number.trim().parse::<usize>() {
            Ok(task_number) => {
                if task_number > 0 && task_number <= self.task_list.len() {
                    let removed_task = self.task_list.remove(task_number - 1);
                    println!("Success: Removed task '{:?}'", removed_task);
                } else {
                    println!("Error Invalid Task Number");
                }
            }
            Err(_) => {
                println!("Error: Please Enter a valid number");
            }
        }
    }
    fn view_task(&self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        } else {
            for (index, task) in self.task_list.iter().enumerate() {
                println!("{}. {:?}", index + 1, task);
            }
        }
        println!("-------------------------");
    }

    fn edit_task(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks available to edit!");
            return;
        }
        self.view_task();

        print!("Please enter the task number to edit: ");
        io::stdout().flush().unwrap();

        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid Input");

        match task_number.trim().parse::<usize>() {
            Ok(task_number) => {
                if task_number > 0 && task_number <= self.task_list.len() {
                    let task = &mut self.task_list[task_number - 1];

                    print!(
                        "Enter new description (leave blank to keep '{}'): ",
                        task.description
                    );
                    io::stdout().flush().unwrap();
                    let mut description = String::new();
                    io::stdin()
                        .read_line(&mut description)
                        .expect("Invalid input");

                    let description = description.trim();
                    if !description.is_empty() {
                        task.description = description.to_string();
                    }

                    print!(
                        "Enter new priority (1-5) or press Enter to keep '{}': ",
                        task.priority
                    );
                    io::stdout().flush().unwrap();
                    let mut priority = String::new();
                    io::stdin().read_line(&mut priority).expect("Invalid input");

                    if let Ok(priority_val) = priority.trim().parse::<u8>() {
                        if (1..=5).contains(&priority_val) {
                            task.priority = priority_val;
                        } else {
                            println!("Priority not changed: must be 1-5.");
                        }
                    }

                    println!("Success: Task Updated");
                } else {
                    println!("Error: Invalid task number");
                }
            }
            Err(_) => println!("Error: Please enter a valid number"),
        }
    }

    fn view_completed_task(&self) {
        if self.task_list.is_empty() {
            println!("No tasks are editable!");
            return;
        }
        for task in &self.task_list {
            if task.completed == true {
                println!("{:?}", task);
            }
        }
    }
    fn view_pending_task(&self) {
        if self.task_list.is_empty() {
            println!("No tasks found!");
            return;
        }
        for task in &self.task_list {
            if task.completed == false {
                println!("{:?}", task);
            }
        }
    }
    fn mark_complete_task(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks found!");
            return;
        }
        print!("Please enter the task no. to Mark Complete: ");
        io::stdout().flush().unwrap();

        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid Input");

        match task_number.trim().parse::<usize>() {
            Ok(task_number) => {
                if task_number > 0 && task_number <= self.task_list.len() {
                    self.task_list[task_number - 1].completed = true;
                    println!("Task is completed");
                }
            }
            Err(_) => {
                println!("Error: Please Enter a valid number");
            }
        }
    }
}

fn main() {
    let mut task_list = TaskList::new();
    loop {
        // --- Menu Display ---
        println!("\n--- To-Do List Menu ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Task");
        println!("4. Edit Task");
        println!("5. View completed tasks");
        println!("6. View Pending tasks");
        println!("7. Mark Complete");
        println!("8. Exit");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid Input");

        let choice: u32 = choice.trim().parse().expect("Invalid Number");

        match choice {
            1 => task_list.add_task(),
            2 => task_list.view_task(),
            3 => task_list.remove_task(),
            4 => task_list.edit_task(),
            5 => task_list.view_completed_task(),
            6 => task_list.view_pending_task(),
            7 => task_list.mark_complete_task(),
            8 => {
                println!("Exiting ...");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}
