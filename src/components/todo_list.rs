use yew::prelude::*;
use super::todo_item::TodoItem;
use super::app::Todo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub todos: Vec<Todo>,
    pub on_toggle: Callback<usize>,
    pub on_delete: Callback<usize>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &Props) -> Html {
    html! {
        <ul class="todo-list">
            {props.todos.iter().map(|todo| {
                let id = todo.id;
                html! {
                    <TodoItem
                        key={id}
                        todo={todo.clone()}
                        on_toggle={props.on_toggle.reform(move |_| id)}
                        on_delete={props.on_delete.reform(move |_| id)}
                    />
                }
            }).collect::<Html>()}
        </ul>
    }
}