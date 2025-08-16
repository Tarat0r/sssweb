use yew::prelude::*;
use super::sss_encryption::Encryption;
use super::sss_decryption::Decryption;
use super::information::Information;

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Encrypt,
    Decrypt,
    Information,
}

#[function_component(TabMenu)]
pub fn tab_menu() -> Html {
    let active_tab = use_state(|| Tab::Encrypt);
    let threshold = use_state(|| "".to_string());
    let share_count = use_state(|| "".to_string());

    let onclick = {
        let active_tab = active_tab.clone();
        move |tab: Tab| {
            let active_tab = active_tab.clone();
            Callback::from(move |_| active_tab.set(tab))
        }
    };

    let on_threshold = {
        let threshold = threshold.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(num) = input.value().parse::<u32>() {
                    if num > 1 {
                        threshold.set(num.to_string());
                    }
                }
            }
        })
    };

    let on_share_count = {
        let share_count = share_count.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(num) = input.value().parse::<u32>() {
                    if num > 1 {
                        share_count.set(num.to_string());
                    }
                }
            }
        })
    };

    // Parse to numbers (0 means "invalid / not set" here)
    let t_num = (*threshold).parse::<u32>().unwrap_or(0);
    let s_num = (*share_count).parse::<u32>().unwrap_or(0);

    let inputs_valid = t_num > 1 && s_num > 1 && t_num <= s_num;

    html! {
        <div>
            // Tab buttons
            <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
                <button onclick={onclick(Tab::Encrypt)}>{ "Encrypt" }</button>
                <button onclick={onclick(Tab::Decrypt)}>{ "Decrypt" }</button>
                <button onclick={onclick(Tab::Information)}>{ "Information" }</button>
            </div>
            <hr />

            <div>
                {
                    match *active_tab {
                        Tab::Information => html! {
                            <Information />
                        },
                        _ => html! {
                            <>
                                // Shared input
                                <div>
                                    <label for="shared-input">{ "Threshold: " }</label>
                                    <input type="number" min="2" value={(*threshold).clone()}  oninput={on_threshold} placeholder="Enter threshold" />
                                    <br />
                                    <label for="share-count">{ "Number of Shares: " }</label>
                                    <input type="number" min="2" value={(*share_count).clone()} oninput={on_share_count} placeholder="Enter number of shares" />
                                </div>
                                // Tab-specific logic
                                {
                                    match *active_tab {
                                        Tab::Encrypt => {                                           
                                            if inputs_valid {
                                                html! { <Encryption threshold={t_num} share_count={s_num} /> }
                                            } else {
                                                html! { <p>{ "Enter valid numbers (>1) for both fields and threshold must be less than or equal to share count." }</p> }
                                            }
                                        },
                                        Tab::Decrypt => {
                                            if t_num > 1 {
                                                html! { <Decryption threshold={t_num} /> }
                                            } else {
                                                html! { <p>{ "Enter a valid threshold (>1)." }</p> }
                                            }
                                        },
                                        _ => html! {},
                                    }
                                }
                            </>
                        }
                    }
                }
            </div>
        </div>
    }
}
