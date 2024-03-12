use sycamore::prelude::*;

#[component]
pub async fn Fight<G: Html>() -> View<G> {
    let fighters = create_signal(vec!["🐱", "🐵", "🐶"]);
    let fighter: Signal<Option<&str>> = create_signal(None);

    view! {
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *fighters,
                    view = move |f| view! {
                        li {
                            button(on:click=move |_| fighter.set(Some(f))) { (f) }
                        }
                    }
                )
            }
            p {
                "you chose: "
                (fighter.get().unwrap_or("nothing"))
            }
        }
    }
}
