use yew::prelude::*;

#[function_component(Information)]
pub fn information() -> Html {
    html! {
        <section class="sss-guide">
            <p>
                <strong>{"Shamir's secret sharing:"}</strong>
                {" Split a secret into "}
                <code>{"n"}</code>
                {" pieces so that any "}
                <code>{"t"}</code>
                {" pieces can rebuild it, while fewer than "}
                <code>{"t"}</code>
                {" reveal nothing at all. Great for keys, recovery codes, and high‑stakes passwords."}
            </p>

            <hr/>

            <h2>{"Why people use it"}</h2>
            <ul>
                <li><strong>{"Stronger safety:"}</strong>{" Nobody holds the whole secret alone."}</li>
                <li><strong>{"Availability:"}</strong>{" Lose a few shares? Recovery still works."}</li>
                <li><strong>{"Flexible policy:"}</strong>{" Choose "}<code>{"t"}</code>{"/"}<code>{"n"}</code>{" to fit personal or team risk."}</li>
                <li><strong>{"Easy rotation:"}</strong>{" If a share leaks, re‑issue new shares without changing the underlying secret."}</li>
            </ul>

            <hr/>

            <h2>{"Mental model"}</h2>
            <p>{"Imagine a smooth curve drawn so it passes through chosen dots. Each share is one dot (a tiny labeled point). With "}<code>{"t"}</code>{" dots, there’s exactly one curve that fits — and the secret is embedded in that curve. With fewer than "}<code>{"t"}</code>{", there are countless possible curves, so the secret stays hidden. (In practice this happens inside a finite number system designed to prevent leaks.)"}</p>

            <hr/>

            <h2>{"How it works — quick flow"}</h2>
            <ol>
                <li><strong>{"Choose a policy:"}</strong>{" decide "}<code>{"t"}</code>{" (threshold) and "}<code>{"n"}</code>{" (number of shares)."}</li>
                <li><strong>{"Encode the secret:"}</strong>{" treat it as bytes/a number."}</li>
                <li><strong>{"Generate shares:"}</strong>{" a randomized process outputs "}<code>{"n"}</code>{" labeled shares bound to the secret."}</li>
                <li><strong>{"Recover later:"}</strong>{" provide any "}<code>{"t"}</code>{" valid shares to reconstruct the secret; fewer than "}<code>{"t"}</code>{" are useless."}</li>
            </ol>

            <hr/>

            <h2>{"Good defaults & use cases"}</h2>
            <ul>
                <li><strong>{"Common policies:"}</strong>{" 2‑of‑3 (personal backup), 3‑of‑5 (small team), 5‑of‑8 or 5‑of‑9 (org leadership)."}</li>
                <li><strong>{"Use it for:"}</strong>{" break‑glass access to a vault/HSM/admin account; crypto‑wallet recovery; distributing a password manager’s master key among teammates."}</li>
            </ul>

            <hr/>

            <h2>{"Where Reed–Solomon fits"}</h2>
            <p>{"Both SSS and Reed–Solomon codes evaluate polynomials at different points. Reed–Solomon uses this to detect/correct errors in data; SSS uses it so any "}<code>{"t"}</code>{" points reveal the hidden value while fewer points reveal nothing."}</p>

            <hr/>

            <h2>{"Learn more"}</h2>
            <ul>
                <li>
                    <strong>{"Visual walk‑through (recommended): "}</strong>
                    <em>{"Shamir’s Secret Sharing: Explanation and Visualization"}</em>
                    {" — Evervault (cool animations) — "}
                    <a href="https://evervault.com/blog/shamir-secret-sharing" target="_blank" rel="noopener noreferrer">{"evervault.com/blog/shamir-secret-sharing"}</a>
                </li>
                <li>
                    <strong>{"Concise overview: "}</strong>
                    {"Wikipedia — "}
                    <em>{"Shamir’s secret sharing"}</em>
                    {" — "}
                    <a href="https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing" target="_blank" rel="noopener noreferrer">{"wikipedia.org/wiki/Shamir%27s_secret_sharing"}</a>
                </li>
                <li>
                    <strong>{"Deeper dive (math): "}</strong>
                    <em>{"Horcrux: Implementing SSS in Rust (part 1) — Guillaume Endignoux"}</em>
                    {" — "}
                    <a href="https://gendignoux.com/blog/2021/11/01/horcrux-1-math.html" target="_blank" rel="noopener noreferrer">{"gendignoux.com/blog/2021/11/01/horcrux-1-math.html"}</a>
                </li>
                <li>
                    <strong>{"Video, SSS made easy: "}</strong>
                    <em>{"How to keep an open secret with mathematics"}</em>
                    {" — "}
                    <a href="https://youtu.be/K54ildEW9-Q" target="_blank" rel="noopener noreferrer">{"youtu.be/K54ildEW9-Q"}</a>
                </li>
                <li>
                    <strong>{"Video, background concept: "}</strong>
                    <em>{"What are Reed–Solomon Codes? How computers recover lost data"}</em>
                    {" — "}
                    <a href="https://youtu.be/1pQJkt7-R4Q" target="_blank" rel="noopener noreferrer">{"youtu.be/1pQJkt7-R4Q"}</a>
                </li>
            </ul>

            <hr/>

            <h2>{"Quick FAQ"}</h2>
            <dl>
                <dt><strong>{"Is this encryption?"}</strong></dt>
                <dd>{"Not exactly. It splits control of a secret; you can combine with encryption."}</dd>

                <dt><strong>{"Can I add more shares later?"}</strong></dt>
                <dd>{"Yes — re‑share without changing the underlying secret.(but is not implemented in this tool yet :("}</dd>

                <dt><strong>{"Does it work for files?"}</strong></dt>
                <dd>{"Yes — typically you secret‑share a key that protects, but if the file is large it's better to encrypt the file first (with AES for example) than split the key."}</dd>
            </dl>

            <hr/>

            <h2>{"Glossary"}</h2>
            <ul>
                <li><strong>{"Share:"}</strong>{" one labeled piece given to a person/device."}</li>
                <li><strong>{"Threshold (t):"}</strong>{" minimum number of shares needed to reconstruct."}</li>
                <li><strong>{"Information‑theoretic security:"}</strong>{" holds even against unlimited computing power."}</li>
                <li><strong>{"Interpolation:"}</strong>{" the “connect‑the‑dots” step used during recovery."}</li>
                <li><strong>{"Finite field:"}</strong>{" the safe arithmetic world used under the hood."}</li>
            </ul>

            <hr/>

            </section>
    }
}
