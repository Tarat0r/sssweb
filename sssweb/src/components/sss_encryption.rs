use yew::prelude::*;
use zeroize::{Zeroizing};
use super::copy_button::CopyButton;
const ECC_LEN: usize = 16;

#[derive(Properties, PartialEq)]
pub struct EncryptionProps {
    pub threshold: u32,
    pub share_count: u32,
}

#[function_component(Encryption)]
pub fn sss_encryption(props: &EncryptionProps) -> Html {

    let secret = use_state(|| "A".to_string());
    let parts= use_state(Vec::new);
    let threshold = props.threshold as usize;
    let share_count = props.share_count as usize;

    let secret_closure = secret.clone();
    let parts_closure = parts.clone();

    let secret_input = secret.clone();

    let oninput = {
        let secret_closure = secret_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            secret_closure.set(input.value());
        })
    };

    let onclick =  Callback::from(move |_| {
            let secret_bytes = Zeroizing::new(secret_closure.as_bytes().to_vec());
            let parts_new = shamir_gf256::split(&secret_bytes, threshold, share_count);
            parts_closure.set(parts_new);
            
        });

        let copy_all_text: String = parts
        .iter()
        .map(|share| shamir_gf256::share_to_hex(share, ECC_LEN))
        .collect::<Vec<_>>()
        .join("\n");

        
    html! {
        <div>
            <h2>{ "Encryption" }</h2>
            <p>{ "This is the encryption component." }</p>
            
            <textarea
                placeholder="Secret"
                rows="4"
                cols="30"
                style="width: 100%"
                value={(*secret_input).clone()}
                oninput={oninput}
            />
            <p>{ format!("Secret: {:?}", secret_input.as_str())}</p>
            <details>
                <summary>{ "View HEX" }</summary>
                <code>{ format!("HEX: {:02x?}", secret_input.as_bytes())}</code>
            </details>
            <button {onclick}>{ "Go!" }</button>
            <br />
            <br />
            <div>
            { (!parts.is_empty()).then(|| html! {
                <>
                <CopyButton text={copy_all_text.clone()} />
                <br />
                </>
            })}

            {
                for copy_all_text.lines().enumerate().map(|(i, line)| html! {
                    <>
                    <p class="prevent-select">{ format!("Share {}: ", i + 1)}</p>
                    <code>{ format!("{line}") }</code>
                    <br />
                    </>
            })
        }
            </div>
        </div>
    }
}
