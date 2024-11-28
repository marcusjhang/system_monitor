use sysinfo::{System, SystemExt, ProcessorExt, ProcessExt, DiskExt}; // Ensure these traits are included
use walkdir::WalkDir;
use std::{thread, time};

fn display_progress_bar(label: &str, value: f32) {
    let bar_length = 50;
    let filled_length = (bar_length as f32 * value / 100.0) as usize;
    let progress = "=".repeat(filled_length) + &" ".repeat(bar_length - filled_length);
    println!("{}: [{}] {:.2}%", label, progress, value);
}

fn display_battery_info() {
    // Placeholder for battery information
    println!("Battery Info: Not Available");
}

fn display_process_info(system: &System) {
    // Display the top 3 memory-consuming processes
    let mut processes: Vec<_> = system.processes().iter().collect();
    processes.sort_by_key(|&(_, p)| p.memory());

    println!("\nTop 3 Memory Consuming Processes:");
    for (pid, process) in processes.iter().take(3) {
        println!(
            "PID: {} | {} | Memory: {} KB",
            pid,
            process.name(),
            process.memory()
        );
    }
}

fn display_file_explorer() {
    let mut current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/"));

    loop {
        // Display the current directory
        println!("\nCurrent Directory: {}", current_dir.display());
        
        // Use walkdir to list files and directories
        let files = WalkDir::new(&current_dir)
            .max_depth(2) // Limit the depth of directories to explore
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir() || e.file_type().is_file())
            .collect::<Vec<_>>();

        // Print the files and directories
        for entry in files {
            let file_type = if entry.file_type().is_dir() {
                "[DIR]  "
            } else {
                "[FILE] "
            };
            println!("{}{}", file_type, entry.path().display());
        }

        // User input for navigation
        println!("\nCommands: ");
        println!("  'cd <dir>' to change directory");
        println!("  'ls' to list files");
        println!("  'exit' to quit the file explorer");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input == "exit" {
            break;
        } else if input.starts_with("cd ") {
            let new_dir = input.trim_start_matches("cd ").trim();
            let new_path = current_dir.join(new_dir);
            if new_path.exists() && new_path.is_dir() {
                current_dir = new_path;
            } else {
                println!("Directory not found: {}", new_dir);
            }
        } else if input == "ls" {
            // Already displayed the files above
        } else {
            println!("Unknown command: {}", input);
        }
    }
}

fn main() {
    let mut system = System::new_all(); // Initialize the system object to get all info

    loop {
        // Ask user what to display: system stats or file explorer
        println!("\nSystem Health Monitor and File Explorer");
        println!("1. Show System Health Stats");
        println!("2. Enter File Explorer");
        println!("Enter your choice (1 or 2): ");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        match choice.trim() {
            "1" => {
                // System Stats
                println!("\nSystem Health Monitor");
                println!("------------------------");
                system.refresh_all();

                let cpu_usage: f32 = system.processors().iter().map(|p| p.cpu_usage()).sum();
                let total_memory = system.total_memory();
                let used_memory = system.used_memory();
                
                // Handling the potential overflow during disk space calculation
                let total_disk = system.disks().iter().map(|d| d.total_space()).sum::<u64>();
                let used_disk = system.disks().iter().map(|d| {
                    let available_space = d.available_space();
                    // Ensure that available space is never greater than total space
                    if available_space > d.total_space() {
                        0 // Avoid overflow, return zero usage if this happens
                    } else {
                        d.total_space() - available_space
                    }
                }).sum::<u64>();

                display_progress_bar("CPU Usage", cpu_usage);
                println!("Memory Usage: {}/{} KB", used_memory, total_memory);
                display_progress_bar("Disk Usage", (used_disk as f32 / total_disk as f32) * 100.0);

                display_process_info(&system);

                println!("\nRefreshing system stats in 2 seconds...");
                thread::sleep(time::Duration::from_secs(2));
            }
            "2" => {
                // File Explorer
                display_file_explorer();
            }
            _ => {
                println!("Invalid choice, please enter 1 or 2.");
            }
        }
    }
}