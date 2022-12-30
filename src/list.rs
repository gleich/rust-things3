use std::collections::HashMap;

use anyhow::{Context, Result};
use osascript::JavaScript;

use crate::todo::Todo;

#[derive(Debug)]
pub enum List {
    Inbox,
    Today,
    Anytime,
    Upcoming,
    Someday,
}

impl ToString for List {
    fn to_string(&self) -> String {
        match self {
            List::Inbox => String::from("TMInboxListSource"),
            List::Today => String::from("TMTodayListSource"),
            List::Anytime => String::from("TMNextListSource"),
            List::Upcoming => String::from("TMCalendarListSource"),
            List::Someday => String::from("TMSomedayListSource"),
        }
    }
}

impl List {
    pub fn fetch_todos(&self) -> Result<Vec<Todo>> {
        let script = JavaScript::new(
            "
        const things = Application('Things3');
        const todos = things.lists.byId($params.list).toDos();
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
        let mut parameters = HashMap::new();
        parameters.insert("list", self.to_string());
        let todos: Vec<Todo> = script
            .execute_with_params(parameters)
            .context("executing osascript to fetch todos failed")?;
        Ok(todos)
    }
}
