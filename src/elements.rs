use gloo::utils::document;
use web_sys::{Element, Node};
use yew::prelude::*;
use yew::{html, virtual_dom::AttrValue};

#[function_component]
pub fn MyComponent() -> Html {
    let node = use_memo(
        |_| {
            let div: Element = document().create_element("div").unwrap();
            div.set_inner_html("hello, world");
            let node: Node = div.into();
            Html::VRef(node)
        },
        (),
    );

    (*node).clone()
}

#[function_component(DynamicTagName)]
pub fn dynamic_tag_name() -> Html {
    let level = 5;
    let text = "Hello World Dynamic Tag Name".to_owned();

    html! {
        <@{format!("h{}", level)} class="title">{text}</@>
    }
}

#[function_component]
pub fn StringLikeAttr() -> Html {
    let str_placeholder = "I'm a str!";
    let string_placeholder = String::from("I'm a String!");
    let attrvalue_placeholder = AttrValue::from("I'm an AttrValue!");

    html! {
        <>
            <input placeholder={str_placeholder} />
            <input placeholder={string_placeholder} />
            <input placeholder={attrvalue_placeholder} />
        </>
    }
}
