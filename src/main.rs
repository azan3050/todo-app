use std::io::stdin;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("What would you like to do");
        println!("1. Add a Todo item");
        println!("2. update Todo item");
        println!("3. Display Todo item");
        println!("$. Quit");

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim().parse::<u32>().expect("Invalid Inout");

        match choice {
            1 => {
                println!("Enter the name of Todo Item: ");
                let mut name = String::new();
                stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            },

            2 => {
                println!("Enter the ID of Todo Item: ");
                let mut id = String::new();
                stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            },

            3 => {
                display_items(&todo_list);
            },

            4 => {
                println!("GoodBye!");
                return;
            },

            _ => {
                println!("Invalid choice");
            },
        }
    }
}
fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}