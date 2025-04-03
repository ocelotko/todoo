# TODO CLI Application in Rust

This is a simple todo command-line application built using the Rust programming language. It allows you to manage your tasks directly from your terminal.

## Features

* **Add tasks:** Easily add new tasks to your todo list.
* **List tasks:** View all your current tasks with their completion status.
* **Toggle task completion:** Mark tasks as done or undone.
* **Remove tasks:** Delete tasks from your list.
* **Persistence:** Your tasks are saved to a `tasks.json` file in the same directory, so they persist between sessions.

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Building and Running

1.  Clone the repository:
    ```bash
    git clone https://github.com/ocelotko/todoo.git
    cd todoo
    ```

2.  Build the application:
    ```bash
    cargo build --release
    ```

3.  Run the application (from the project root):
    ```bash
    ./target/release/todoo <command> [arguments]
    ```

## Usage

The todo application accepts the following commands:

* `add <task_description>`: Adds a new task with the given description.
    ```bash
    ./target/release/todoo add "Buy groceries"
    ```

* `list`: Lists all the tasks in your todo list with their status and index.
    ```bash
    ./target/release/todoo list
    ```
    Output example:
    ```
    Tasks:
    1. [ ] Buy groceries
    2. [X] Walk the dog
    3. [ ] Finish report
    ```

* `toggle <task_index>`: Toggles the completion status of the task at the given index. Use the index shown by the `list` command.
    ```bash
    ./target/release/todoo toggle 1
    ```

* `remove <task_index>`: Removes the task at the given index. Use the index shown by the `list` command.
    ```bash
    ./target/release/todoo remove 2
    ```

* Running the application without any command or with an unknown command will display a usage message.
