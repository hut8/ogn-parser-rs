
ogn-parser
==============================================================================

![Build Status Core](https://github.com/Meisterschueler/ogn-parser-rs/actions/workflows/core.yaml/badge.svg)
![Build Status Pyo3](https://github.com/Meisterschueler/ogn-parser-rs/actions/workflows/pyo3.yaml/badge.svg)

Development
------------------------------------------------------------------------------

### Setting up pre-commit hooks

This project uses [pre-commit](https://pre-commit.com/) to run the same checks locally that run in CI (cargo fmt, cargo clippy, and cargo test).

To install pre-commit:

```bash
# Using pip
pip install pre-commit

# Or using your system package manager
# Ubuntu/Debian:
sudo apt install pre-commit
# macOS:
brew install pre-commit
```

Then install the git hooks:

```bash
pre-commit install
```

Now the checks will run automatically on every commit. To run the checks manually:

```bash
# Run on all files
pre-commit run --all-files

# Run on staged files only
pre-commit run
```

License
------------------------------------------------------------------------------

This project is licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)

 - MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.
