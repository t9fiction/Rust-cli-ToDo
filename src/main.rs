use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    completed: bool,
    description: String,
}

impl Task {
    fn new(id: i32, name: String, description: String) -> Task {
        Task {
            id,
            name,
            completed: false,
            description,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

#[derive(Debug)]
struct TodoList{
    tasks: Vec<Task>,
    id_counter: i32,
}

impl  TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            id_counter: 1,
        }
    }

    fn add_task(&mut self, name: &str, description: &str) {
        let task = Task::new(self.id_counter, name.to_string(), description.to_string());
        self.tasks.push(task);
        self.id_counter += 1;
    }

    fn complete_task(&mut self, id: i32){
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id){
            task.mark_completed();
        } else {
            println!("Task with id {} not found", id);
        }
    }

    // Display the todo list in a table format
    fn display(&self) {
        println!("{:<5} {:<20} {:<10} {}", "ID", "Name", "Completed", "Description");
        println!("{:-<5} {:-<20} {:-<10} {:-<30}", "", "", "", "");
        for task in &self.tasks {
            let status = if task.completed { "Yes" } else { "No" };
            println!("{:<5} {:<20} {:<10} {}", task.id, task.name, status, task.description);
        }
        println!("{:-<5} {:-<20} {:-<10} {:-<30}", "", "", "", "");
    }

    fn display_task(&self, id: i32) {
        if let Some(task) = self.tasks.iter().find(|task| task.id == id) {
            let status = if task.completed { "Yes" } else { "No" };
            println!("\nTask ID: {}\nName: {}\nCompleted: {}\nDescription: {}\n", task.id, task.name, status, task.description);
        } else {
            println!("Task with id {} not found", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\nMenu:");
        println!("Enter 1, 2, 3, 4 or 0");
        println!("1 - Add a task");
        println!("2 - Mark task as completed");
        println!("3 - Display all tasks");
        println!("4 - Display a specific task");
        println!("0 - Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice{
            "1" => {
                let mut name = String::new();
                let mut description = String::new();

                print!("Enter task name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();

                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim();

                todo_list.add_task(name, description);
                todo_list.display();
            }
            "2" => {
                let mut id_str = String::new();
                print!("Enter task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    todo_list.complete_task(id);
                    todo_list.display();
                } else {
                    println!("Invalid task ID");
                }
            }
            "3" => {
                todo_list.display();
            }
            "4" => {
                let mut id_str = String::new();
                print!("Enter task ID to display: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    todo_list.display_task(id);
                } else {
                    println!("Invalid task ID");
                }
            }
            "0" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid input. Please choose a valid option.");
            }
        }
    }
}
