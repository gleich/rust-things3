use things3::list::List;

fn main() {
    let todos = List::Today
        .fetch_todos()
        .expect("Failed to get todos for today");
    for todo in todos {
        println!("{}", todo.name);
    }
}
