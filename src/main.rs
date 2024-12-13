mod task;
mod storage;

use crate::task::Task;
use crate::storage::{save_tasks_to_file, load_tasks_from_file};
use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let filename = "tasks.json";

    loop {
        println!("\n--- Меню ---");
        println!("1. Показати завдання");
        println!("2. Створити завдання");
        println!("3. Вибрати завдання");
        println!("4. Завантажити завдання з файлу");
        println!("5. Зберегти завдання в файл");
        println!("0. Вихід");

        let choice = get_input("Виберіть дію: ");

        match choice.trim() {
            "1" => show_tasks(&tasks),
            "2" => create_task(&mut tasks),
            "3" => select_task(&mut tasks),
            "4" => {
                match load_tasks_from_file(filename) {
                    Ok(loaded_tasks) => {
                        tasks = loaded_tasks;
                        println!("Завдання завантажено успішно.");
                    }
                    Err(e) => eprintln!("Помилка при завантаженні завдань: {}", e),
                }
            }
            "5" => {
                if let Err(e) = save_tasks_to_file(&tasks, filename) {
                    eprintln!("Помилка при збереженні завдань: {}", e);
                } else {
                    println!("Завдання збережено успішно.");
                }
            }
            "0" => {
                println!("До побачення!");
                break;
            }
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}

// Функція для вводу даних
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

// Функція для показу всіх завдань
fn show_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("Немає завдань.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "Виконано" } else { "Не виконано" };
            println!("{}. {} - {} ({})", index + 1, task.title, task.description, status);
        }
    }
}

// Функція для створення нового завдання
fn create_task(tasks: &mut Vec<Task>) {
    let title = get_input("Введіть назву завдання: ");
    let description = get_input("Введіть опис завдання: ");
    let new_task = Task::new(
        tasks.len() as u32 + 1, 
        title.trim().to_string(),
        description.trim().to_string(),
    );
    tasks.push(new_task);
    println!("Завдання додано.");
}

// Функція для вибору завдання і виконання дій
fn select_task(tasks: &mut Vec<Task>) {
    let index_str = get_input("Введіть номер завдання для редагування: ");
    let index: usize = match index_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Невірний номер завдання.");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Такого завдання не існує.");
        return;
    }

    let task = &mut tasks[index - 1];
    
    // Виведення вибраного завдання
    println!("\nВибране завдання:");
    println!("Назва: {}", task.title);
    println!("Опис: {}", task.description);
    let status = if task.completed { "Виконано" } else { "Не виконано" };
    println!("Статус: {}", status);

    loop {
        // Меню для вибраного завдання
        println!("\n--- Меню завдання ---");
        println!("1. Відмітити як виконане");
        println!("2. Відредагувати завдання");
        println!("3. Видалити завдання");
        println!("0. Повернутися до головного меню");

        let choice = get_input("Виберіть дію: ");

        match choice.trim() {
            "1" => {
                task.mark_as_completed();
                println!("Завдання відмічене як виконане.");
                break;
            }
            "2" => {
                let new_title = get_input("Введіть нову назву завдання: ");
                let new_description = get_input("Введіть новий опис завдання: ");
                task.title = new_title.trim().to_string();
                task.description = new_description.trim().to_string();
                println!("Завдання відредаговане.");
                break;
            }
            "3" => {
                tasks.remove(index - 1);
                println!("Завдання видалено.");
                break;
            }
            "0" => break,
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}
