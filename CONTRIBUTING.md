# Contributing to aleo-cli

Thank you for considering contributing to `aleo-cli`! By participating in this project, you help improve the Aleo blockchain development experience for CLI users worldwide.

## How to Contribute

### 1. Reporting Bugs

If you find a bug, please:

- Check the [issues](https://github.com/qiaopengjun5162/aleo-cli/issues) to see if it has already been reported.
- If not, create a new issue with:
  - A clear description of the problem.
  - Steps to reproduce the bug.
  - Any relevant logs or error output.

### 2. Suggesting Features

Have an idea for a new feature or improvement?

- Search existing issues to see if your idea is already suggested.
- If not, open a new issue with a clear description, why it's valuable, and any additional context.

### 3. Submitting Code Changes

1. Fork the repository to your GitHub account.
2. Create a new branch: `feature/your-feature` or `fix/bug-description`.
3. Make your changes in this branch.
4. Ensure all tests pass and the code is properly linted:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

5. Commit with clear, concise messages following [Conventional Commits](https://www.conventionalcommits.org/):

   ```
   feat: add support for private key import
   fix: resolve panic on empty balance response
   docs: update README with deploy example
   test: add integration tests for transfer command
   ```

6. Push to your fork and create a Pull Request to the `main` branch.

### 4. Code Style

- Follow the [Rust style guide](https://doc.rust-lang.org/book/ch01-01-installation.html).
- Run `cargo fmt` before committing.
- Keep imports grouped: standard library → external crates → local modules.
- Aim for meaningful test coverage on new code.

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/aleo-cli.git
cd aleo-cli

# Build
cargo build

# Run tests
cargo test

# Run lints
cargo clippy --all-targets -- -D warnings
```

## Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to enforce code quality before commits:

```bash
# Install pre-commit and hooks
pip install pre-commit
pre-commit install --hook-type pre-commit --hook-type commit-msg --hook-type pre-push
```

The hooks verify:
- **commit-msg**: Conventional commit message format
- **pre-commit**: `cargo fmt`, `cargo clippy`, `cargo check`
- **pre-push**: `cargo test`

## License

By contributing, you agree that your contributions will be licensed under the MIT License (see [LICENSE](LICENSE)).
