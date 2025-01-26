use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    created_at: DateTime<Local>,
    is_complete: bool,
}

#[derive(Parser)]
#[command(name = "tasks")]
#[command(about = "A CLI tool to manage your tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
    },
    List {
        #[arg(short, long)]
        all: bool,
    },
    Complete {
        task_id: u32,
    },
    Delete {
        task_id: u32,
    },
}

const FILE_PATH: &str = "tasks.csv";

fn open_file() -> io::Result<File> {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        File::create(path)
    } else {
        OpenOptions::new().read(true).write(true).open(path)
    }
}

fn write_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(open_file()?);
    for task in tasks {
        wtr.serialize(task)?;
    }
    wtr.flush()?;
    Ok(())
}

fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }
    let mut rdr = csv::Reader::from_path(FILE_PATH)?;
    rdr.deserialize()
        .collect::<Result<Vec<Task>, _>>()
        .map_err(From::from)
}

fn add_task(description: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;
    let id = tasks.last().map_or(1, |t| t.id + 1);
    let new_task = Task {
        id,
        description,
        created_at: Local::now(),
        is_complete: false,
    };
    tasks.push(new_task);
    write_tasks(&tasks)?;
    println!("Task added successfully!");
    Ok(())
}

fn list_tasks(show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = read_tasks()?;
    for task in tasks.iter().filter(|t| show_all || !t.is_complete) {
        println!(
            "ID: {}\tTask: {}\tCreated: {}\tDone: {}",
            task.id, task.description, task.created_at, task.is_complete
        );
    }
    Ok(())
}

fn complete_task(task_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.is_complete = true;
        write_tasks(&tasks)?;
        println!("Task marked as complete!");
    } else {
        eprintln!("Task with ID {} not found.", task_id);
    }
    Ok(())
}

fn delete_task(task_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != task_id);

    if tasks.len() < initial_len {
        write_tasks(&tasks)?;
        println!("Task deleted successfully!");
    } else {
        eprintln!("Task with ID {} not found.", task_id);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => add_task(description)?,
        Commands::List { all } => list_tasks(all)?,
        Commands::Complete { task_id } => complete_task(task_id)?,
        Commands::Delete { task_id } => delete_task(task_id)?,
    }

    Ok(())
}
