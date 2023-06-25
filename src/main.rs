use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Paula Brillant!" }</h1>
            <footer>
                <a href="mailto:generated@ebobo.dev">{"generated@ebobo.dev"}</a>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // TODO: check csr dependency.
}
