# Design parity checker

This is the repository for the tool "dpc", Design Parity Checker. The goal of the tool is to provide a utility for comapring the likeness of two images representing an implemented design of an app, webapp or website.

This makes it possible to automatically evaluate how precise an LLM (or human) has been in their implementation work against a reference.

## Goals of the project

## Non-goals of the project

## Codebase structure

- `src/` Rust crate. Entrypoints `src/main.rs` (bin) + `src/lib.rs` (lib).
- `src/commands/` CLI command handlers.
- `src/browser/` browser automation + screenshots.
- `src/figma/` + `src/figma_client.rs` Figma integration.
- `src/metrics/` similarity metrics + scoring logic.
- `src/pipeline.rs` orchestration pipeline; `src/output.rs` result formatting.
- `src/types/` + `src/types.rs` shared types.
- `tests/` integration + CLI tests.
- `test_assets/` fixture images + data.
- `docs/` project docs + findings.

## Agent protocol

- No destructive git commands.
- Always commit your work.
- Multiple agents are working here; ignore changes you don't know about. Workdir will be dirty, expected.

## Tools

- `sv task` for task tracking; `sv task --robot-help`.
- `fmail` for agent coordination.

## 
