# Exercism Rust Workspace

This workspace contains a collection of Rust exercises from the [Exercism](https://exercism.org/tracks/rust) Rust track. Each directory represents a standalone project designed to teach specific Rust concepts.

## 🛠️ Project Structure

The workspace is organized into individual exercise directories:

- **`anagram/`**: Practice with strings, sorting, and lifetimes.
- **`clock/`**: Implementing a type with specific arithmetic and display behavior.
- **`flower-field/`**: (Likely a custom or specific exercise, needs investigation if complex).
- **`luhn/`**: Algorithm implementation and string parsing.
- **`space-age/`**: Working with Traits, Macros, and unit conversions.
- **`sublist/`**: List comparison and pattern matching.

Each exercise directory typically contains:
- `src/lib.rs`: The main implementation file (often starts as a stub).
- `tests/*.rs`: Integration tests to verify the solution.
- `Cargo.toml`: Project configuration and dependencies.
- `README.md` & `HELP.md`: Exercise-specific instructions and troubleshooting.

## 🚀 Building and Running

Since each directory is an independent Cargo project, you must navigate into the specific exercise directory to run commands.

### Testing
To run the tests for an exercise:
```bash
cd <exercise-directory>
cargo test
```
*Note: Some exercises may have ignored tests by default. Use `cargo test -- --ignored` to run all tests.*

### Building & Checking
To compile the code or check for errors without running tests:
```bash
cargo build
cargo check
```

## 💡 Development Conventions

- **Surgical Implementation**: Most exercises require implementing logic in `src/lib.rs`. Follow the provided function signatures and types.
- **Idiomatic Rust**: Prioritize idiomatic solutions (e.g., using iterators, pattern matching, and proper error handling).
- **Dependencies**: Exercism's online test runner has a restricted list of available crates. Check the comment in each `Cargo.toml` for the link to the allowed dependencies.
- **Version**: Projects are currently using the `2024` edition of Rust.

## 🧪 Testing Strategy

- **Test-Driven Development**: The provided tests are the source of truth for completion.
- **Incremental Progress**: It is common to un-ignore tests one by one as you implement the required functionality.
- **Verification**: A solution is considered complete when all tests pass (including ignored ones).
