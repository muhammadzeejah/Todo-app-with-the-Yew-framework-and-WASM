use yew::prelude::*;
use super::todo_input::TodoInput;
use super::todo_list::TodoList;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| {
        LocalStorage::get::<Vec<Todo>>("todos").unwrap_or_else(|_| Vec::new())
    });

    let next_id = use_state(|| {
        todos.iter().map(|todo: &Todo| todo.id).max().unwrap_or(0) + 1
    });

    let on_add = {
        let todos = todos.clone();
        let next_id = next_id.clone();
        Callback::from(move |text: String| {
            let mut new_todos = (*todos).clone();
            new_todos.push(Todo {
                id: *next_id,
                text,
                completed: false,
            });
            LocalStorage::set("todos", &new_todos).unwrap();
            todos.set(new_todos);
            next_id.set(*next_id + 1);
        })
    };

    let on_toggle = {
        let todos = todos.clone();
        Callback::from(move |id: usize| {
            let mut new_todos = (*todos).clone();
            if let Some(todo) = new_todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
                LocalStorage::set("todos", &new_todos).unwrap();
                todos.set(new_todos);
            }
        })
    };

    let on_delete = {
        let todos = todos.clone();
        Callback::from(move |id: usize| {
            let mut new_todos = (*todos).clone();
            new_todos.retain(|t| t.id != id);
            LocalStorage::set("todos", &new_todos).unwrap();
            todos.set(new_todos);
        })
    };

    let on_clear_all = {
        let todos = todos.clone();
        Callback::from(move |_: MouseEvent| {
            LocalStorage::delete("todos");
            todos.set(Vec::new());
        })
    };

    html! {
        <div class="container">
            <h1 class="header">{"Todo App"}</h1>
            <TodoInput {on_add} />
            <TodoList 
                todos={(*todos).clone()} 
                {on_toggle}
                {on_delete}
            />
            <button class="clear-all" onclick={on_clear_all}>
                {"Clear All"}
            </button>
        </div>
    }
}