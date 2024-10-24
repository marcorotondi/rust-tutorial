use std::fs;
use std::io::{BufRead, BufReader, Write};

fn main() {
    println!("Welcome to Rust Todo!");
    let mut todos = load_todos().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n1. Add Todo\n2. List Todo\n3. Remove Todo\n4. Quit");
        let choice = read_line();

        match choice.trim() {
            "1" => add_todo(&mut todos),
            "2" => list_todo(&mut todos),
            "3" => remove_todo(&mut todos),
            "4" => break,
            _ => println!("Invalid input, try again"),
        }
    }

    match save_todos(&todos) {
        Ok(_) => println!("Todos Saved!"),
        Err(e) => eprintln!("Failed to save todos: {}", e)
    }
}

fn remove_todo(todos: &mut Vec<String>) {
    list_todo(todos);
    if !todos.is_empty() {
        println!("Enter the number of the todo to remove:");
        if let Ok(num) = read_line().trim().parse::<usize>() {
            if num > 0 && num <= todos.len() {
                todos.remove(num - 1);
                println!("Todo Removed");
            } else {
                println!("Invalid number!");
            }
        }
    }
}

fn list_todo(todos: &[String]) {
    println!("\n############# TODO LIST ##################");
    if todos.is_empty() {
       println!("No todos to show!")
    } else {
        for(index, todo) in todos.iter().enumerate() {
            println!("{}. {}", index + 1, todo);
        }
    }
    println!("##########################################");
}

fn add_todo(todos: &mut Vec<String>) {
    println!("Enter todo: ");
    let todo = read_line();
    if !todo.is_empty() {
        todos.push(todo);
        println!("Todo added!")
    }
}

fn save_todos(todos:&[String]) -> std::io::Result<()> {
    let mut file = fs::File::create("todos.txt").expect("Failed to create file");
    for todo in todos {
        writeln!(file, "{}", todo)?;
    }

    Ok(())
}

fn load_todos() -> std::io::Result<Vec<String>> {
    let file = fs::File::open("todos.txt")?;
    let reader = BufReader::new(file);

    Ok(reader.lines().collect::<Result<Vec<_>, _>>()?)
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

