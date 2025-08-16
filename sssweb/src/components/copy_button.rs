use yew::prelude::*;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct CopyButtonProps {
    pub text: AttrValue,
}

#[function_component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let copied = use_state(|| false);
    let text = props.text.clone();

    let onclick = {
        let copied = copied.clone();
        Callback::from(move |_| {
            if let Some(win) = window() {
                let cb = win.navigator().clipboard();
                // Fire-and-forget: we don’t await the Promise
                let _ = cb.write_text(&text);
                copied.set(true); // optimistic UI
            }
        })
    };

    html! {
        <button {onclick}>
            { if *copied { "Copied!" } else { "Copy" } }
        </button>
    }
}
