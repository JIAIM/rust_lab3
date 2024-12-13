use std::fs::File;
use crate::task::Task;

pub fn save_tasks_to_file(tasks: &Vec<Task>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?; 
    serde_json::to_writer(file, tasks)?;
    Ok(())
}

/// Загрузка задач из файла
pub fn load_tasks_from_file(filename: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?; 
    let tasks: Vec<Task> = serde_json::from_reader(file)?; 
    Ok(tasks)
}
