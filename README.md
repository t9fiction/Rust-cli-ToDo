Here’s a basic README file for your Todo List application written in Rust:

---

# Todo List Application

This is a simple command-line **Todo List** application built with Rust. It allows users to manage tasks by adding, marking as completed, and displaying tasks in a tabular format.

## Features
- **Add a Task**: Adds a new task with a name and description.
- **Mark Task as Completed**: Marks a task as completed.
- **Display All Tasks**: Displays all tasks in a table format.
- **Display a Specific Task**: Display a single task by its ID.
- **Exit**: Exits the program.

## Usage

### Prerequisites
Ensure you have **Rust** installed on your system. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Running the Application

1. Clone the repository:
   ```bash
   git clone [https://github.com/your-username/todo-list.git](https://github.com/t9fiction/Rust-cli-ToDo.git)
   ```

2. Navigate to the project folder:
   ```bash
   cd todo-list
   ```

3. Build and run the application using Cargo (Rust's package manager):
   ```bash
   cargo run
   ```

### Interaction

Upon running the program, the user will be prompted with a menu of options:

1. **Press 1** to add a new task.
2. **Press 2** to mark a task as completed.
3. **Press 3** to display all tasks.
4. **Press 4** to display a specific task by ID.
5. **Press 0** to exit the program.
6. Any other input will be treated as an **invalid input**.

### Sample Menu Interaction

```
Choose an option:
1. Add a Task
2. Mark Task Completed
3. Display All Tasks
4. Display a Specific Task
0. Exit

Enter your choice: 
```

### Display Format

Tasks are displayed in a table with columns for the **ID**, **Name**, **Completed**, and **Description**. Here's an example of the table layout:

```
ID    Name                 Completed  Description                   
----- -------------------- ---------- ------------------------------
1     Task 1               Yes        To learn Rust                  
2     Task 2               No         Complete tutorial on Rust      
----- -------------------- ---------- ------------------------------
```

## Code Explanation

- **Task Struct**: Represents a task with an ID, name, completion status, and description.
- **TodoList Struct**: Holds a list of tasks and a counter to generate task IDs.
- **Methods**:
  - `add_task`: Adds a new task.
  - `complete_task`: Marks a task as completed by its ID.
  - `display`: Displays all tasks in a tabular format.

## Contributing

If you would like to contribute to this project, please fork the repository and create a pull request with your proposed changes.

## License

This project is open-source and available under the MIT License.

---

Feel free to modify this README according to any additional features or specific details you want to include!
