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
        <div class="tab-container">
            // Tab buttons
            <div class="tab-nav">
                <button 
                    class={if *active_tab == Tab::Encrypt { "tab-button active" } else { "tab-button" }}
                    onclick={onclick(Tab::Encrypt)}
                >
                    { "Encrypt" }
                </button>
                <button 
                    class={if *active_tab == Tab::Decrypt { "tab-button active" } else { "tab-button" }}
                    onclick={onclick(Tab::Decrypt)}
                >
                    { "Decrypt" }
                </button>
                <button 
                    class={if *active_tab == Tab::Information { "tab-button active" } else { "tab-button" }}
                    onclick={onclick(Tab::Information)}
                >
                    { "Information" }
                </button>
            </div>

            <div class="tab-content">
                {
                    match *active_tab {
                        Tab::Information => html! {
                            <Information />
                        },
                        _ => html! {
                            <>
                                // Shared input
                                <div class="card">
                                    <div class="form-group">
                                        <label class="form-label" for="threshold-input">{ "Threshold" }</label>
                                        <input 
                                            class="form-input"
                                            id="threshold-input"
                                            type="number" 
                                            min="2" 
                                            value={(*threshold).clone()}  
                                            oninput={on_threshold} 
                                            placeholder="Enter threshold (minimum 2)" 
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label class="form-label" for="share-count-input">{ "Number of Shares" }</label>
                                        <input 
                                            class="form-input"
                                            id="share-count-input"
                                            type="number" 
                                            min="2" 
                                            value={(*share_count).clone()} 
                                            oninput={on_share_count} 
                                            placeholder="Enter number of shares (minimum 2)" 
                                        />
                                    </div>
                                </div>
                                // Tab-specific logic
                                {
                                    match *active_tab {
                                        Tab::Encrypt => {                                           
                                            if inputs_valid {
                                                html! { <Encryption threshold={t_num} share_count={s_num} /> }
                                            } else {
                                                html! { 
                                                    <div class="status-message status-warning">
                                                        { "Enter valid numbers (>1) for both fields and threshold must be less than or equal to share count." }
                                                    </div>
                                                }
                                            }
                                        },
                                        Tab::Decrypt => {
                                            if t_num > 1 {
                                                html! { <Decryption threshold={t_num} /> }
                                            } else {
                                                html! { 
                                                    <div class="status-message status-warning">
                                                        { "Enter a valid threshold (>1)." }
                                                    </div>
                                                }
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
