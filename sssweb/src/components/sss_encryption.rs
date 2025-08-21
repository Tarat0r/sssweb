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
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Encryption"}</h2>
            </div>
            
            <div class="form-group">
                <label class="form-label" for="secret-input">{"Secret to Encrypt"}</label>
                <textarea
                    class="form-input"
                    id="secret-input"
                    placeholder="Enter your secret here..."
                    rows="4"
                    value={(*secret_input).clone()}
                    oninput={oninput}
                />
            </div>

            <div class="form-group">
                <button class="btn btn-primary" {onclick}>
                    {"Generate Shares"}
                </button>
            </div>

            { (!parts.is_empty()).then(|| html! {
                <div class="card">
                    <div class="card-header">
                        <h3 class="card-title">{"Generated Shares"}</h3>
                    </div>
                    <div class="form-group">
                        <CopyButton text={copy_all_text.clone()} />
                    </div>
                    <div class="form-group">
                        {
                            for copy_all_text.lines().enumerate().map(|(i, line)| html! {
                                <div class="share-item">
                                    <p class="share-label">{ format!("Share {}:", i + 1)}</p>
                                    <code class="result-display">{ format!("{line}") }</code>
                                </div>
                            })
                        }
                    </div>
                </div>
            })}
        </div>
    }
}
