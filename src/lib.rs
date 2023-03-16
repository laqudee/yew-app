use yew::prelude::*;

mod nested_children_props;
mod elements;
mod events;
mod lists;
mod contexts;

#[function_component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties" }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    user_first_name: String,
    user_last_name: String,
}

#[derive(Clone, PartialEq, Properties)]
struct Props2 {
    id: String,
    children: Children,
}

#[function_component]
fn Container(props: &Props2) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
fn MyComponentWithProps(props: &Props) -> Html {
    let Props { user_first_name, user_last_name } = props;
    html! {
        <>
            {"user_first_name: "}{user_first_name}{" and user_last_name: "}{user_last_name}
        </>
    }
}

#[function_component]
pub fn ChildrenComponent() -> Html {
    let props = Props {
        user_first_name: "Bob".to_owned(),
        user_last_name: "Smith".to_owned()
    };

    html! {
        <>
            <MyComponent /> <br />

            <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" /> <br />

            <MyComponentWithProps ..props.clone() /> <br />

            <MyComponentWithProps user_last_name ="Elm" ..props /> <br />

            <Container id={"container"}>
                <h4>{"Hi"}</h4>
                <div>{"Hello"}</div>
            </Container>

            <br />
            <nested_children_props::ItWorks />
            <br />
            <elements::MyComponent /> <br />
            <elements::DynamicTagName /> <br />
            <elements::StringLikeAttr /> <br />
            <events::ItWork />
            <div>
                <lists::ListWork />
            </div>
            <div>
                <contexts::App />
            </div>
        </>
    }
}

