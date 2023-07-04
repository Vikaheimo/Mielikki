# Mielikki

A simple file manager that aims to be faster than File Explorer.

## Current Goals

- [x] ~~File lookup in under 10 seconds~~ (Backend searches happen in about 1ms)
- [ ] Windows support
- [x] Directory lookup faster than Windows File Explorer
- [x] Simple Navigable UI
- [ ] Bug free? (Never going to happen)

## Technologies used

Core Dependencies
| Name | Version |
|-|-|
| Svelte-Kit | 1.20 |
| Rust | 1.70 |
| Tauri | 1.4 |

## Development

Developing this project requires cargo and npm. After installing both please install tauri-cli with [these](https://tauri.app/v1/guides/getting-started/prerequisites) instructions. After that the codebase can be run with `cargo tauri run`, or compiled with `cargo tauri build`.

### Issues

If you happen to find a bug, then you can report it on [here](https://github.com/Vikaheimo/Mielikki/issues/new), or by fixing it yourself, by creating a Pull request. Try to be as detailed as possible, so that finding and fixing the bug can be made as easy as possible.
