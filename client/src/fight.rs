use sycamore::prelude::*;

#[component]
pub async fn Fight<G: Html>() -> View<G> {

    let fighters = create_signal(vec!["🐱", "🐵", "🐶"]);

    let choose = move |e| {
    };

    view! {
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable=*fighters,
                    view=|f| view! {
                        li {
                            a(href=format!("https://www.youtube.com/watch?v={f}")) {
                                (f)
                            }
                        }
                    }                )
            }

            button(on:click=choose) { "Choose" }
        }
    }
}