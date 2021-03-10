# rust-playground

Workspace where I play with Rust

## Toolchain installation

1) Install Rust toolchain installer `rustup`

```bash
curl https://sh.rustup.rs -sSf | sh
```

2) Add toolchain to system path (~/.zshrc or ~/.bashrc)

```bash
export PATH="$HOME/.cargo/env:$HOME/.cargo/bin:$PATH"
```

3) Update toolchain

```bash
rustup update
```

4) Check installation

```bash
rustc --version
```

5) Local documentation

```bash
rustup doc
```

## Compilation using `rustc`

1) Go to folder

```bash
cd project_name
```

2) Compile

```bash
rustc main.rs
```

## Compilation using package manager `cargo`

1) Check version

```bash
cargo --version
```

2) Create new project

```bash
cargo new project_name
cd project_name
```

```bash
cargo new --lib lib_project_name
cd lib_project_name
```

3) Build project

```bash
cargo build
```

4) Run project (build and run)

```bash
cargo run
```

5) Check build

Checks the code to make sure it compiles but doesn’t produce an executable.

```bash
cargo check
```

If you’re continually checking your work while writing the code, using cargo check will speed up the process!

6) Build for release

```bash
cargo build --release
```

7) Local dependency documentation

```bash
cargo doc --open
```

8) Running unit tests

```bash
cargo test
```

## Managing dependencies with `crate`

1) Add dependency (ie: `rand`) to `Cargo.toml`

```toml
[dependencies]
rand = "0.7.2"
```

2) Update dependencies in a project

```bash
cargo update
```

## Linter `clippy`

```bash
cargo clippy --all-targets
```

## Formatter `fmt`

```bash
cargo fmt
```

## Running playground tests

Run the following command from the root to run the tests in this repository:

```sh
cargo run
```