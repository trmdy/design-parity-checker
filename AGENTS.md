# Design parity checker

This is dpc, design parity checker.

The goal of this project is to create a tool that allows us to quickly compare two images for how similar they are. The point is to be able to quickly evaluate if an implementation by an LLM or a human is "up-to-spec".

## Codebase structure

- `src/` Rust crate. Main entrypoints `src/main.rs` (bin) + `src/lib.rs` (lib).
- `src/commands/` CLI command handlers.
- `src/browser/` browser automation and screenshots.
- `src/figma/` + `src/figma_client.rs` Figma integration.
- `src/metrics/` similarity metrics + scoring logic.
- `src/pipeline.rs` orchestration pipeline; `src/output.rs` result formatting.
- `src/types/` + `src/types.rs` shared types.
- `tests/` integration + CLI tests.
- `test_assets/` fixture images and data for tests.
- `docs/` project docs + findings.


## Agent protocol

- Always commit your work after completion.
- There are multiple agents working here. There will be times when the working directory is dirty with changes you don't know about. If so, just ignore them and continue with your work.


## Tools

- We use `sv task` for task management. Use `sv task --robot-help`.
- We use `fmail` for agent communication.
