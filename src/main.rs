use web_sys::*;
use yew::prelude::*;
use fingerprint::WhoAmI;

mod fingerprint;

#[function_component(App)]
fn app() -> Html {
    let window = window().unwrap();
    let document = window.document().unwrap();

    const NAME: &str = "ebobo.dev";

    document.set_title(NAME);

    let email = format!("generated@{}", NAME);

    html! {
        <div>
            <h1>{ NAME } </h1>
            <WhoAmI />
            <footer>
                <a href={ format!("mailto:{}", email) }>{ email }</a>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
