use yew::prelude::*;
mod components;
use components::tab_menu::TabMenu;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <div class="header">
                <h1>{ "🔐 Shamir's Secret Sharing" }</h1>
                <p>{ "This is a demo of Shamir's Secret Sharing over GF(256)" }</p>
                <div class="header-features">
                    <span class="feature-badge">{"🔒 Secure"}</span>
                    <span class="feature-badge">{"⚡ Fast"}</span>
                    <span class="feature-badge">{"🌐 Web-based"}</span>
                </div>
            </div>
            <TabMenu />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
