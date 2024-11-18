use web_sys::HtmlInputElement;
use yew::prelude::*;
use web_sys::MouseEvent;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_add: Callback<String>,
}

#[function_component(TodoInput)]
pub fn todo_input(props: &Props) -> Html {
    let input_ref = use_node_ref();

    let on_click = {
        let input_ref = input_ref.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.trim().is_empty() {
                    on_add.emit(value);
                    input.set_value("");
                }
            }
        })
    };

    let on_keypress = {
        let input_ref = input_ref.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let value = input.value();
                    if !value.trim().is_empty() {
                        on_add.emit(value);
                        input.set_value("");
                    }
                }
            }
        })
    };

    html! {
        <div class="todo-input">
            <input 
                ref={input_ref}
                type="text"
                placeholder="Add a new todo..."
                onkeypress={on_keypress}
            />
            <button onclick={on_click}>{"Add"}</button>
        </div>
    }
}