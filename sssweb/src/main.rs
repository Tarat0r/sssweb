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
                    <span class="feature-badge">{"🦀 Rust + WASM (Yew)"}</span>
                    <span class="feature-badge">{"🔧 Reed-Solomon ECC"}</span>
                    <span class="feature-badge">{"🖥️ Client-side only"}</span>
                    <a class="feature-badge"
                        href="https://github.com/Tarat0r/sssweb"
                        target="_blank"
                        rel="noopener noreferrer">
                        <svg class="icon" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true" focusable="false">
                            <path fill-rule="evenodd" d="M8 .198a8 8 0 00-2.53 15.6c.4.074.55-.174.55-.388l-.01-1.37c-2.24.486-2.71-1.08-2.71-1.08-.364-.924-.89-1.17-.89-1.17-.727-.5.055-.49.055-.49.804.056 1.227.826 1.227.826.715 1.224 1.874.87 2.33.665.072-.518.28-.87.508-1.07-1.79-.2-3.67-.895-3.67-3.98 0-.88.314-1.6.826-2.17-.083-.202-.358-1.016.078-2.12 0 0 .673-.215 2.205.83a7.68 7.68 0 012.005-.27c.68 0 1.37.092 2.01.27 1.53-1.045 2.2-.83 2.2-.83.438 1.104.163 1.918.08 2.12.514.57.825 1.29.825 2.17 0 3.09-1.88 3.78-3.68 3.98.288.25.54.74.54 1.49l-.01 2.2c0 .214.15.463.55.387A8 8 0 008 .198z"/>
                        </svg>
                        { " GitHub repo" }
                    </a>
                </div>
            </div>
            <TabMenu />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
