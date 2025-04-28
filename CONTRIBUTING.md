# Contributing to LightShuttle

Welcome! 👋  
Thank you for considering contributing to **LightShuttle**.

We are building a lightweight, self-hostable orchestrator for containerized applications.  
Contributions of any size are welcome: bug fixes, features, tests, documentation, ideas.

---

## Getting Started

First, fork the repository and clone it locally:

```bash
git clone https://github.com/your-username/lightshuttle.git
cd lightshuttle
```

Install the necessary tools:

```bash
cargo install cargo-make
```

---

## Building the Project

To check and build everything:

```bash
make
```

This runs:

- `cargo fmt` to format the code
- `cargo clippy` to lint
- `cargo build` to compile

You can also run LightShuttle locally:

```bash
make run
```

Or using Docker:

```bash
make docker-up
```

---

## Running Tests

Run the full test suite with:

```bash
make test
```

---

## Code Style

Please follow the project's coding standards:

- Rust stable
- Code formatted with `cargo fmt`
- No Clippy warnings (`-D warnings`)
- Public functions and structs must have a doc comment (`///`)

We use `cargo-make` for task automation.

---

## Opening a Pull Request

- Make sure `make ci` passes.
- Provide a clear description of your changes.
- Link related issues if applicable.
- Try to keep PRs small and focused.

---

## Communication

If you want to propose a bigger change, open an issue first to discuss it.

---

## License

By contributing, you agree that your contributions will be licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**.
