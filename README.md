# yew-app

> [Yew Document](https://yew.rs/docs/getting-started/introduction)

## 准备

### 安装 WebAssembly target

```shell
rustup target add wasm32-unknown-unknown
```

### 安装 Trunk

```shell
cargo install --locked trunk
```

### Other options

- wasm-pack
- wasm-run

## Build app

1. `cargo new yew-app`

2. Cargo.toml

```toml
[package]
name = "yew-app"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.84"
yew = { version = "0.20.0", features = ["csr"] }
gloo = "0.8.0"
yew-router = "0.17.0"

[dependencies.web-sys]
version = "0.3.61"

features = [
    "console",
    "Document",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]
```

3. `trunk serve`

## Yew concepts

- Basic Web Technologies
- Component
- HTML
- Agent
- Context
- Router
- Suspense

- Struct Component
- Chidlren
- Optimozations
- Portals
- Server-side Rendering
- Immutable Types
