use yew::prelude::*;

#[function_component]
pub fn ListWork() -> Html {
    let items = (1..=10).collect::<Vec<_>>();

    html! {
        <>
            <ul class="item-list">
                { items.iter().collect::<Html>() }
            </ul>
            <ul class="item-list">
                { for items.iter() }
            </ul>
            <KeyedList />
        </>
    }
}

#[function_component]
fn KeyedList() -> Html {
    let names = vec!["Sam","Bob","Ray"];

    let list = html! {
        <div id="introuction">
            {
                names.into_iter().map(|name| {
                    html! {<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
                }).collect::<Html>()
            }
        </div>
    };

    let show_link = true;
    let maybe_display_link = move || -> Html {
        if show_link {
            html! {
                <a href="https://example.com">{"Link"}</a>
            }
        } else {
            html! {}
        }
    };

    html! {
        <>
            {list} 
            <div>
                {maybe_display_link()}
            </div>
        </>
    }
}