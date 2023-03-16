use wasm_bindgen::JsCast;
use web_sys::console::log_1;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
        <label for="cautious-input">
                { "My cautious input:" }
                <input onchange={on_cautious_change}
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <label for="dangerous-input">
                { "My dangerous input:" }
                <input onchange={on_dangerous_change}
                    id="dangerous-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </>
    }
}

#[function_component]
pub fn ItWork() -> Html {
    html! {
        <>
            <button onclick={Callback::from(|_| (log_1(&("点击事件").into())))}>{"点击"}</button>
            <div>
                <MyComponent />
            </div>
        </>
    }
}
