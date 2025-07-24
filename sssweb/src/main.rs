
use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html{
    let state = use_state(|| Model{
        value: 0
    });

    let secret = use_state(|| "A".to_string());
    let parts = use_state(Vec::new);
    let result = use_state(Vec::new);
    
    let threshold = 3;
    let share_count = 5;
    
    let parts_closure = parts.clone();
    let secret_closure = secret.clone();
    let result_closure: UseStateHandle<Vec<u8>> = result.clone();

    let secret_input = secret.clone();

    let oninput = {
        let secret_closure = secret_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            secret_closure.set(input.value());
        })
    };

    let onclick =  Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            });
            let secret_bytes = secret_closure.as_bytes().to_vec();
            let parts_new = shamir_gf256::split(&secret_bytes, threshold, share_count);
            parts_closure.set(parts_new.clone()); 
            result_closure.set(shamir_gf256::reconstruct(&parts_new, threshold));
        });
    

    html! {
        <div>
            <h1>{ "Shamir's Secret Sharing" }</h1>
            <p>{ "This is a demo of Shamir's Secret Sharing over GF(256)." }</p>
            <hr />
            <input type="text" placeholder="Secret" value={(*secret_input).clone()} oninput={oninput}/>
            <p><b>{ format!("Secret: {:?} ( as bytes {:?} )", secret_input.as_str(), secret_input.as_bytes()) }</b></p>
            <p>{ format!("Threshold: {}", threshold) }</p>
            <p>{ format!("Share count: {}", share_count) }</p>
            <hr />
            <p>{ format!("Secret share shards: {:?}", parts) }</p>
            <hr />
            <p>{ format!("Reconstructed secret: {:?}", result) }</p>

            <button {onclick}>{ "Go!" }</button>
        </div>
    }
}

fn main(){
    yew::Renderer::<App>::new().render();
}