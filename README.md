# Mostra

**Mostra** is an open-source, local-first engine for discovering, classifying
and managing personal data exposure.

It is designed as an independent reusable security/privacy core rather than as
Silens-specific application logic.

## Workspace

```text
crates/
├── mostra/       canonical Rust core
├── mostra-ffi/   future C ABI
├── mostra-wasm/  future WebAssembly ABI
└── mostra-cli/   native command-line interface
```

The initial **0.0.1** release publishes only the core `mostra` crate.

The C ABI, WebAssembly and CLI packages remain unpublished until their public
contracts are ready.

## Architecture principles

- local-first by default
- no personal source data upload requirement
- deterministic behaviour
- privacy-preserving processing
- application-agnostic core
- minimal dependencies
- Rust as canonical implementation
- semantic consistency across Rust, C and WebAssembly interfaces
- CLI as a first-class application surface

## Ecosystem role

Mostra is intended to power exposure-analysis applications while remaining
usable independently as a library or command-line engine.

Silens Expose is one planned consumer of Mostra. Mostra itself does not depend
on Silens.

## Status

**0.0.1 — initial architecture bootstrap**

No production exposure-analysis API is claimed by this release.

## License

MIT
