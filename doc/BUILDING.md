# Building

This document describes how to build the project binaries, API documentation, and the manual.

## Build the project

From the repository root:

```bash
cargo build
```

To build with optimizations:

```bash
cargo build --release
```

## Build the API documentation

From the repository root:

```bash
cargo doc --no-deps
```

The generated API docs are written to:

```text
target/doc/
```

## Build the manual (PDF + HTML)

The manual source files are read from:

```text
doc/manual/en/??-*.md
```

### Prerequisites

- `pandoc`
- Eisvogel template available to Pandoc (`--template eisvogel`)
- A LaTeX engine supported by your Pandoc setup (for PDF output)

### Build command

From the repository root:

```bash
./doc/build_manual.sh
```

The generated manual files are written to:

```text
target/doc/pgmoneta-mcp-en.pdf
target/doc/pgmoneta-mcp-en.html
```
