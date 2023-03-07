use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = Callback::from(
        move |_| counter.set(*counter + 1)
    );

    html! {
        <div>
            <h1>{"Hello World!"}</h1>
            <button {onclick}>{"Click me!"}</button>
            <p> { *counter }</p>
        </div>
    }
}


fn main() {
    yew::Rendered::<App>::new().render();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
