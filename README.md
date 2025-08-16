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

---

## 🗂️ Project layout

```bash
.
├── Cargo.toml
├── README.md
├── shamir-gf256
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── sssweb
    ├── Cargo.toml
    ├── dist/              # Built static assets (output of `trunk build`)
    ├── index.html         # Trunk entry-point
    └── src
        └── main.rs
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
