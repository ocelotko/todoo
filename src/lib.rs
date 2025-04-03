use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

const TASK_FILE: &str = "tasks.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    name: String,
    completed: bool,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            name: name.to_string(),
            completed: false,
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn save_to_file(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        std::fs::write(TASK_FILE, json)?;
        Ok(())
    }

    pub fn load_from_file() -> Result<Self> {
        if std::path::Path::new(TASK_FILE).exists() {
            let data = std::fs::read_to_string(TASK_FILE)?;
            let task_list: TaskList = serde_json::from_str(&data)?;
            Ok(task_list)
        } else {
            Ok(TaskList { tasks: Vec::new() })
        }
    }

    pub fn add_task(&mut self, args: &[String]) {
        if let Some(name) = args.first() {
            let task = Task::new(name);
            self.tasks.push(task);
        } else {
            print!("No task name provided");
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            let task_index = index - 1;
            self.tasks.remove(task_index);
        }
    }

    pub fn toggle_task_completion(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            let task_index = index - 1;
            if let Some(task) = self.tasks.get_mut(task_index) {
                task.toggle_completed();
            }
        }
    }

    pub fn list_tasks(&self) {
        println!("Tasks:");
        if self.tasks.is_empty() {
            println!("No tasks");
            return;
        }

        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[X]" } else { "[ ]" };
            println!("{}. {} {}", index + 1, status, task.name);
        }
    }
}
