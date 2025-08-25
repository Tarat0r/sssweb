# Shamir’s Secret Sharing (GF256) — Rust + Yew WASM Demo

A small Rust **workspace** containing:

- **`shamir-gf256`** — a minimal, educational implementation of Shamir’s Secret Sharing (SSS) over **GF(256)**.
- **`sssweb`** — a Yew/WebAssembly front‑end that demonstrates splitting and reconstructing secrets in the browser.

> ⚠️ **Security disclaimer:** this repository is intended for learning and experimentation. It has **not** been audited. The math is sound, but the implementation choices (e.g., randomness source, timing behavior, input handling) have not been hardened for production cryptography.

---

## ✨ Features

- GF(256) arithmetic with the AES irreducible polynomial **x⁸ + x⁴ + x³ + x + 1 (0x11B)**.
- Byte‑wise SSS: each secret byte is protected by its own random polynomial.
- Clean, idiomatic Rust with `Add/Sub/Mul/Div` operators on `GF256`.
- `Zeroize` on sensitive types (`Share`) to reduce memory remanence risk.
- Yew + `wasm-bindgen` browser UI (served by Trunk) to play with SSS.
- Helpful unit tests and `Display` impls for debugging.
- Reed–Solomon–compatible layout (shares are polynomial evaluations over GF(256)); see notes on error correction below.

---

## 🗂️ Project layout

```bash
.
├── Cargo.toml
├── .github/workflows
│       └── ci.yaml
├── .gitignore
├── README.md
├── shamir-gf256
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       └── share_codec.rs
└── sssweb
    ├── Cargo.toml
    ├── dist
    ├── index.html
    ├── src
    │   ├── components
    │   │   ├── copy_button.rs
    │   │   ├── information.rs
    │   │   ├── mod.rs
    │   │   ├── sss_decryption.rs
    │   │   ├── sss_encryption.rs
    │   │   └── tab_menu.rs
    │   └── main.rs
    └── style.css

9 directories, 21 files

```

---

## 🚀 Getting started

### Prerequisites

- Rust (stable) & Cargo  
- **wasm32** target for Rust:

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **Trunk** for building/serving the Yew app:

  ```bash
  cargo install trunk
  ```

### Build & test the library

```bash
# At repo root
cargo build -p shamir-gf256
cargo test  -p shamir-gf256
```

### Run the web app (dev)

```bash
cd sssweb
trunk serve --open
```

Trunk compiles the Yew app to WebAssembly and serves it at a local URL (it will print the address in your terminal and open a browser tab).

### Build the web app (release / static files)

```bash
cd sssweb
trunk build --release
# Outputs to sssweb/dist (you can host these static files anywhere)
```

---

## 📦 Library overview (`shamir-gf256`)

The core types and functions:

- `GF256(u8)` — field element with +, −, ×, ÷ over GF(256).
- `Share { x: GF256, y: Vec<u8> }` — a share at x with a y‑vector the same length as the secret.
- `split(secret: &[u8], threshold: usize, share_count: usize) -> Vec<Share>`  
  Split a secret into `share_count` parts; any `threshold` of them can reconstruct.
- `reconstruct(shares: &[Share], threshold: usize) -> Vec<u8>`  
  Rebuild the secret using Lagrange interpolation at `x = 0`.

> Limits: `share_count <= 255` and `2 <= threshold <= 255` (with `threshold <= share_count`).  
> Internally, x‑coordinates are `1..=share_count` as `GF256` elements.

### Quick example

```rust
use shamir_gf256::{split, reconstruct, Share};

fn main() {
    let secret = b"correct horse battery staple";
    let threshold = 3;
    let share_count = 5;

    // Split
    let parts: Vec<Share> = split(secret, threshold, share_count);

    // Distribute parts[..] to participants...

    // Reconstruct from any 3 parts:
    let recovered = reconstruct(&parts[0..3], threshold);
    assert_eq!(recovered, secret);
}
```

### Tests included

- `gf_add_mul`: checks basic field math against known values.
- `round_trip`: splits and reconstructs a small secret and validates equality.

Run them with:

```bash
cargo test -p shamir-gf256
```

---

## 🌐 Web UI overview (`sssweb`)

A lightweight Yew app that imports the library and offers a browser UI for:

- choosing a **threshold** and **number of shares**
- entering a **secret** (bytes/text)
- splitting into shares and reconstructing from a subset
- (optional) copy-to-clipboard helpers

> The app is set up for Trunk with `index.html` including:
>
> - `<link data-trunk rel="rust" />` to compile the Rust crate to WASM
> - an optional Plotly script tag (used if you add charts/visualizations)

### Serving in development

```bash
cd sssweb
trunk serve --open
```

### Hosting the built app

Any static host works. After `trunk build --release`, upload the contents of `sssweb/dist/` to your host (Netlify, GitHub Pages, S3, a simple Nginx, …).

---

## 🧩 Reed–Solomon / error correction

Shamir’s Secret Sharing evaluates a random degree-k-1 polynomial over GF(256) at distinct x points. That is a systematic Reed–Solomon (RS) codeword structure with parameters RS(n = share_count, k = threshold).
This means you can, in principle, correct corrupted shares (errors) or recover from missing ones (erasures) using standard RS decoders.

**Current status:** this repository’s reconstruct expects exact shares and does not ship an RS decoder yet (e.g., Berlekamp–Welch/Sugiyama–Berlekamp–Massey + Forney). Contributions welcome.

### What error/erasure budgets look like

If you create n shares with threshold k, classical RS bounds give:

- Up to t errors and e erasures provided 2t + e ≤ n - k.
- Special cases:
  - Only erasures (known-missing or flagged-bad): recover if e ≤ n - k.
  - Only errors (unknown-bad shares): correct if t ≤ ⌊(n - k)/2⌋.

**Example:** k = 3, n = 7 ⇒ n - k = 4.
You can correct 2 errors, or 4 erasures, or mixes like 1 error + 2 erasures.

### How it would integrate here

- Each secret byte position is an independent RS(n, k) symbol stream over GF(256).
- We already fix x = 1..=n (as GF(256) elements), which is standard for RS decoding.
- An RS decoder would run per-byte across the selected shares and output the polynomial’s value at x=0 (the secret byte).

### Authenticity vs. robustness

RS decoding gives robustness to random noise or a few malicious shares within bounds. It does not authenticate shares. For adversarial settings use a MAC (e.g., compute a MAC of the secret and share it too) or a verifiable secret sharing (VSS) scheme/commitments.

### Practical tips (until RS is implemented)

- Generate some redundancy: pick n > k so you can discard a few bad/missing shares.
- Add integrity checks to each share payload (e.g., length + CRC32/Blake3 hash) to detect corruption early.

---

## 🔐 Security notes

- **Randomness:** The current code uses `rand::random::<u8>()` when generating polynomial coefficients. For **production‑grade** use, prefer `rand::rngs::OsRng` (or another CSPRNG) and feed randomness explicitly.
- **Timing/side‑channels:** The arithmetic is not guaranteed constant‑time.
- **Zeroization:** `Share` implements `Zeroize` & `ZeroizeOnDrop`, but you should still be mindful about where secrets live in memory (e.g., browser consoles, logs, crashes).
- **Validation:** Inputs should be strictly validated in any real application (lengths, ranges, encodings).

---

## 🛠️ Troubleshooting

- **Trunk not found:** `cargo install trunk`
- **WASM target missing:** `rustup target add wasm32-unknown-unknown`
- **Port in use:** `trunk serve --port 8081`
- **Missing modules:**  
  If you see compile errors about `share_codec` (in `shamir-gf256`) or `components::tab_menu` (in `sssweb`), ensure those files exist or temporarily remove the `mod`/`use` lines until the modules are added.

---

## 📚 References

- [Arithmetic background inspired by Guillaume Endignoux’s write‑up on Horcrux math (GF(256) with the AES polynomial).](https://gendignoux.com/blog/2021/11/01/horcrux-1-math.html)

---
