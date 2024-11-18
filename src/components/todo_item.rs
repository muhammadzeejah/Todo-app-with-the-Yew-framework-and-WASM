use yew::prelude::*;
use super::app::Todo;
use web_sys::MouseEvent;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub todo: Todo,
    pub on_toggle: Callback<usize>,
    pub on_delete: Callback<usize>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &Props) -> Html {
    let text_class = if props.todo.completed {
        "text completed"
    } else {
        "text"
    };

    let on_toggle = {
        let on_toggle = props.on_toggle.clone();
        let id = props.todo.id;
        Callback::from(move |_: MouseEvent| {
            on_toggle.emit(id);
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let id = props.todo.id;
        Callback::from(move |_: MouseEvent| {
            on_delete.emit(id);
        })
    };

    html! {
        <li class="todo-item">
            <input 
                type="checkbox"
                checked={props.todo.completed}
                onclick={on_toggle}
            />
            <span class={text_class}>
                {&props.todo.text}
            </span>
            <button onclick={on_delete}>{"Delete"}</button>
        </li>
    }
}