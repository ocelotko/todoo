use anyhow::Result;
use todoo::TaskList;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut task_list = match TaskList::load_from_file() {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error loading tasks from file: {}", e);
            TaskList { tasks: Vec::new() } // Fallback to an empty list
        }
    };

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => task_list.add_task(&args[2..]),
        Some("remove") => {
            if let Some(index_str) = args.get(2) {
                if let Ok(index) = index_str.parse::<usize>() {
                    task_list.remove_task(index);
                } else {
                    println!("Invalid index: {}", index_str);
                }
            } else {
                println!("No index provided");
            }
        }
        Some("list") => task_list.list_tasks(),
        Some("toggle") => {
            if let Some(index_str) = args.get(2) {
                if let Ok(index) = index_str.parse::<usize>() {
                    task_list.toggle_task_completion(index);
                } else {
                    println!("Invalid index: {}", index_str);
                }
            } else {
                println!("No index provided");
            }
        }
        Some("help") => print_help(),
        _ => print_help(),
    }

    task_list.save_to_file()?;

    Ok(())
}

fn print_help() {
    println!("Usage: todoo <command> [arguments]");
    println!("Commands:");
    println!("  add <description>...  Add a new task.");
    println!("  remove <index>          Remove the task at the given index.");
    println!("  list                    List all tasks.");
    println!(
        "  toggle <index>          Toggle the completion status of the task at the given index."
    );
    println!("  help                    Show this help message.");
}
