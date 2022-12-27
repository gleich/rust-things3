use chrono::{DateTime, Local};
use osascript::JavaScript;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub tags: String,
    pub due_date: Option<DateTime<Local>>,
    pub project: Option<Area>,
    pub area: Option<Area>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: String,
    pub area: Area,
}

#[derive(Debug, Deserialize)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub tags: String,
}

fn main() {
    let script = JavaScript::new(
        "
        const things = Application('Things3');
        const todos = things.lists.byId('TMTodayListSource').toDos();
        return todos.map(todo => ({
            id: todo.id(),
            name: todo.name(),
            status: todo.status(),
            notes: todo.notes(),
            tags: todo.tagNames(),
            due_date: todo.dueDate() && todo.dueDate().toISOString(),
            project: todo.project() && {
            id: todo.project().id(),
            name: todo.project().name(),
            tags: todo.project().tagNames(),
            area: todo.project().area() && {
                id: todo.project().area().id(),
                name: todo.project().area().name(),
                tags: todo.project().area().tagNames(),
            },
            },
            area: todo.area() && {
            id: todo.area().id(),
            name: todo.area().name(),
            tags: todo.area().tagNames(),
            },
        }));",
    );
    let rv: Vec<Todo> = script.execute().expect("Failed to execute script");
    dbg!(rv);
}
