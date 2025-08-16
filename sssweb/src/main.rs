
use yew::prelude::*;
mod components;
use components::tab_menu::TabMenu;

#[function_component(App)]
fn app() -> Html{
    

    html! {
        <div>
            <h1>{ "Shamir's Secret Sharing" }</h1>
            <p>{ "This is a demo of Shamir's Secret Sharing over GF(256)." }</p>
            <hr />
            <TabMenu />
        </div>
    }
}

fn main(){
    yew::Renderer::<App>::new().render();
}