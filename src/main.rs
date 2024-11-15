use std::collections::HashMap;

struct TodoItem {
    description: String,
    completed: bool,
}

struct TodoList {
    items: HashMap<u32, TodoItem>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_item(&mut self, description: &str) {
        let item = TodoItem {
            description: description.to_string(),
            completed: false,
        };
        self.items.insert(self.next_id, item);
        self.next_id += 1;
    }

    fn view_items(&self) {
        for (id, item) in self.items.iter() {
            let status = if item.completed { "✓" } else { "✗" };
            println!("[{}] {}", status, item.description);
        }
    }

    fn mark_complete(&mut self, id: u32) {
        if let Some(item) = self.items.get_mut(&id) {
            item.completed = true;
        }
    }

    fn delete_item(&mut self, id: u32) {
        self.items.remove(&id);
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_item("Buy groceries");
    todo_list.add_item("Clean the house");
    todo_list.add_item("Call mom");

    todo_list.view_items();

    todo_list.mark_complete(2);
    todo_list.delete_item(3);

    todo_list.view_items();
}