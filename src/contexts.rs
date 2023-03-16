use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    theme: Theme,
}

#[function_component]
fn Navbar(props: &NavbarProps) -> Html {
    html! {
        <div>
            <Title theme={props.theme.clone()}>{ "App title" }</Title>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    theme: Theme,
    children: Children,
}

#[function_component]
fn Title(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

#[function_component]
fn NavButton() -> Html {
    // let theme = use_context::<Theme>();
    html! {
        <div>{"这是NavButton"}</div>
    }
}

#[function_component]
pub fn App() -> Html {
    let theme = use_memo(|_| Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
    }, ());

    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            <NavButton />
        </ContextProvider<Rc<Theme>>>
    }
}
