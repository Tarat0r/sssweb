use yew::prelude::*;
use yew::UseStateHandle;
use zeroize::{Zeroizing};

use shamir_gf256::{share_from_hex, reconstruct, Share};

#[derive(Properties, PartialEq)]
pub struct DecryptionProps {
    pub threshold: u32,
}

#[function_component(Decryption)]
pub fn sss_decryption(props: &DecryptionProps) -> Html {
    let threshold = props.threshold as usize;

    // Raw text pasted/typed by the user (each line = one share hex)
    let shares_text = use_state(String::new);

    // Parsed shares and the reconstructed secret
    let parts: UseStateHandle<Vec<Share>> = use_state(Vec::new);
    let result: UseStateHandle<Zeroizing<Vec<u8>>> =
        use_state(|| Zeroizing::new(Vec::new()));    let parse_errors: UseStateHandle<Vec<String>> = use_state(Vec::new);

    // Keep text in sync with the textarea
    let oninput = {
        let shares_text = shares_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            shares_text.set(input.value());
        })
    };

    // Clone state handles for the onclick closure
    let shares_text_closure = shares_text.clone();
    let parts_closure = parts.clone();
    let result_closure = result.clone();
    let errors_closure = parse_errors.clone();

    let onclick = Callback::from(move |_| {
        // Parse: one hex-encoded share per line (from `share_to_hex`)
        let mut parsed: Vec<Share> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (idx, line) in shares_text_closure.lines().enumerate() {
            let l = line.trim();
            if l.is_empty() { continue; }
            match share_from_hex(l) {
                Ok(sh) => parsed.push(sh),
                Err(e) => errors.push(format!("Line {}: {}", idx + 1, e)),
            }
        }

        errors_closure.set(errors);
        parts_closure.set(parsed.clone());

        if parsed.len() >= threshold && threshold > 0 {
            // NOTE: reconstruct expects the first `threshold` shares.
            let reconstructed = reconstruct(&parsed, threshold);
            result_closure.set(Zeroizing::new(reconstructed));
        } else {
            result_closure.set(Zeroizing::new(Vec::new()));
        }
    });

    html! {
        <div>
            <h2>{ "Decryption" }</h2>
            <p>{ "Paste your shares below — one per line." }</p>

            <label for="shares">{ "Shares (one hex blob per line):" }</label>
            <textarea
                id="shares"
                placeholder="Example (per line):
53485231012a00000068656c6c6f
53485231ff0a000000736f6d657061796c6f6164"
                rows="9"
                style="width: 100%;"
                value={(*shares_text).clone()}
                oninput={oninput}
            />
            if shares_text.is_empty() {
                <p>{ format!("Threshold: {}", threshold) }</p>
            } else {}
            <button {onclick}>{ "Reconstruct" }</button>

            {
                if !parse_errors.is_empty() {
                    html! {
                        <>
                            <hr />
                            <h3>{ "Parse errors" }</h3>
                            <ul>
                                { for parse_errors.iter().map(|e| html!{ <li>{ e }</li> }) }
                            </ul>
                        </>
                    }
                } else {
                    html! {}
                }
            }

            if !result.is_empty() {
                <div>
                    <hr />
                    <h3>{ "Reconstructed secret" }</h3>
                    <p>{ format!("{}", String::from_utf8_lossy(&result)) }</p>
                    <details>
                        <summary>{ "View HEX" }</summary>
                        <code>{ format!("HEX: {:02x?}", *result) }</code>
                    </details>
                </div>
            } else {}
        </div>
    }
}
