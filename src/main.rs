use web_sys::{console, HtmlElement, MouseEvent};
use yew::prelude::*;
use yew::{classes, html, Callback, Html, Properties, TargetCast};
// use wasm_bindgen::{prelude::Closure, JsCast};
use gloo::events::EventListener;
use gloo::utils::window;
use std::fmt::Display;
use std::mem::drop;
use yew_app::ChildrenComponent;
use yew_router::prelude::*;

mod router;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(ShowStorageChange)]
pub fn show_storage_change() -> Html {
    let state_storage_changed = use_state(|| false);
    {
        let state_storage_changed = state_storage_changed.clone();
        use_effect(|| {
            let listener = EventListener::new(&window(), "storage", move |_| {
                state_storage_changed.set(true)
            });

            move || drop(listener)
        });
    }

    html! {
        <div>{"Storage Event Fired: "}{*state_storage_changed}</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    data: T,
}

#[function_component]
fn MyGenericComponent<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    html! {
        <p>
            { &props.data}
        </p>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let my_header: Html = html! {
        <div>
            <p>{"这是header"}</p>
            <img src="test.jpg" alt="Girl in a jacket" width="500" height="600" />
        </div>
    };
    let count = 5;
    let counter_html: Html = html! {
        <>
            <p class={classes!("class-1", "class-2")}>{"My age is: "}{count}</p>
            <div class={classes!(String::from("class-1 class-2"))}>{"String::from"}</div>
            <div class={classes!(Some("class"))}>{"Option Some"}</div>
            <div class={classes!(vec!["class-1", "class-2"])}>{"vec!"}</div>
            <div class={classes!(["class-1", "class-2"].as_ref())}>{"as_ref()"}</div>
        </>
    };

    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_x() as f64) - rect.top();
            console::log_1(&format!("Left? : {}; Top?: {}", x, y).into());
        }
    });

    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>

            <router::NavItems />
            <div>
                <div class={classes!("header")}>
                    {my_header}
                </div>
                <button {onclick} style="color: red; font-size: 16px;">{"+1"}</button>
                <p>{ *counter }</p>
            </div>
            <div>
                {counter_html}
            </div>
            <div id="mousemoveme" {onmousemove} style="width: 400px;height: 300px; background: gray;"></div>
            <div>
                <HelloWorld />
            </div>
            <div>
                <ShowStorageChange />
            </div>
            <div>
                <MyGenericComponent<i32> data=123 />
                {"-------------------"}
                <MyGenericComponent<String> data={"foo".to_string()} />
            </div>
            <ChildrenComponent />
        </>
    }
}

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello Wrold" }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
