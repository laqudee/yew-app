use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    value: String,
}

#[function_component]
pub fn ListItem(props: &ListItemProps) -> Html {
    let ListItemProps { value } = props.clone();
    html! {
        <span>{ value }</span>
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

#[function_component]
fn List(props: &Props) -> Html {
    let modified_children = props.children.iter().map(|mut item| {
        let mut props = Rc::make_mut(&mut item.props);
        props.value = format!("item-{} \n", props.value);
        item
    });
    html! {for modified_children }
}

#[function_component]
pub fn ItWorks() -> Html {
    html! {
        <List>
            <ListItem value="a" />
            <ListItem value="b" /> 
            <ListItem value="c" />
        </List>
    }
}
