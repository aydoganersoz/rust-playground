# Installation

1) Install Rust toolchain installer `rustup`
```
curl https://sh.rustup.rs -sSf | sh
```

2) Add toolchain to system path (~/.zshrc or ~/.bashrc)
```
export PATH="$HOME/.cargo/env:$HOME/.cargo/bin:$PATH"
```

3) Update toolchain
```
rustup update
```

4) Check installation
```
rustc --version
```

5) Local documentation
```
rustup doc
```

# Compilation using `rustc`

1) Go to folder
```
cd project_name
```

2) Compile
```
rustc main.rs
```

# Compilation using package manager `cargo`

1) Check version
```
cargo --version
```

2) Create new project
```
cargo new project_name
cd project_name
```

3) Build project
```
cargo build
```

4) Run project (build and run)
```
cargo run
```

5) Check build

Checks the code to make sure it compiles but doesn’t produce an executable.
```
cargo check
```
If you’re continually checking your work while writing the code, using cargo check will speed up the process!

6) Build for release
```
cargo build --release
```

7) Local dependency documentation
```
cargo doc --open
```

# Managing dependencies with `crate`

1) Add dependency (`rand`) to `Cargo.toml`
```
[dependencies]
rand = "0.7.2"
```

2) Update dependencies in a project
```
cargo update
```

# Linter `clippy`
```
cargo clippy --all-targets
```

# Formatter `fmt`
```
cargo fmt
```