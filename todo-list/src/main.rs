use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use clap::{Parser, Subcommand};
use fs2::FileExt;
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
    completed_at: Option<DateTime<Local>>,
    due_date: Option<DateTime<Local>>,
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
        #[arg(short, long)]
        due: Option<String>,
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

fn open_file(truncate: bool) -> io::Result<File> {
    let path = Path::new(FILE_PATH);

    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);

    if truncate {
        options.truncate(true);
    }

    let file = options.open(path)?;

    file.lock_exclusive()?;
    Ok(file)
}

fn write_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(open_file(true)?);
    for task in tasks {
        wtr.serialize(task)?;
    }
    wtr.flush()?;
    Ok(())
}

fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = Path::new(FILE_PATH);

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = open_file(false)?;
    let mut rdr = csv::Reader::from_reader(file);
    rdr.deserialize()
        .collect::<Result<Vec<Task>, _>>()
        .map_err(From::from)
}

fn parse_due_date(input: &str) -> Option<DateTime<Local>> {
    let today = Local::now().date_naive();
    match input.to_lowercase().as_str() {
        "today" => today
            .and_hms_milli_opt(23, 59, 59, 999)?
            .and_local_timezone(Local)
            .single(),
        "tomorrow" => (today + chrono::Duration::days(1))
            .and_hms_milli_opt(23, 59, 59, 999)?
            .and_local_timezone(Local)
            .single(),
        _ => None,
    }
}

fn add_task(description: String, due: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;
    let id = tasks.last().map_or(1, |t| t.id + 1);
    let new_task = Task {
        id,
        description,
        created_at: Local::now(),
        completed_at: None,
        due_date: due.and_then(|d| parse_due_date(&d)),
    };
    tasks.push(new_task);
    write_tasks(&tasks)?;
    println!("Task added successfully!");
    Ok(())
}

fn list_tasks(show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = read_tasks()?;

    if show_all {
        println!(
            "{:<4} {:<50} {:<20} {:<20} {:<20}",
            "ID", "Task", "Created", "Due", "Completed"
        );
    } else {
        println!("{:<4} {:<50} {:<20} {:<20}", "ID", "Task", "Created", "Due");
    }

    for task in tasks
        .iter()
        .filter(|t| show_all || t.completed_at.is_none())
    {
        let created_human_readable = HumanTime::from(task.created_at).to_string();
        let due_human_readable = task
            .due_date
            .map(|d| HumanTime::from(d).to_string())
            .unwrap_or_else(|| "None".to_string());
        let completed_human_readable = task
            .completed_at
            .map(|c| HumanTime::from(c).to_string())
            .unwrap_or_else(|| "Incomplete".to_string());

        if show_all {
            println!(
                "{:<4} {:<50} {:<20} {:<20} {:<20}",
                task.id,
                task.description,
                created_human_readable,
                due_human_readable,
                completed_human_readable
            );
        } else {
            println!(
                "{:<4} {:<50} {:<20} {:<20}",
                task.id, task.description, created_human_readable, due_human_readable
            );
        }
    }
    Ok(())
}

fn complete_task(task_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.completed_at = Some(Local::now());
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
        Commands::Add { description, due } => add_task(description, due)?,
        Commands::List { all } => list_tasks(all)?,
        Commands::Complete { task_id } => complete_task(task_id)?,
        Commands::Delete { task_id } => delete_task(task_id)?,
    }

    Ok(())
}
