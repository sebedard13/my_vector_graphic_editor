# VGC : Concise Graphic Vector

This project reimagined a vector graphic named VGC. At this moment VGC is very limited, but it is a fun experiment to work and explore the subject. With this new format came VGC Editor to manipulate.

## Building

### Dependencies

- Rust with cargo
- cargo-make
- cargo-watch
- wasm-pack
- wasm-bindgen-cli
- tauri
- Node.js
- Angular cli
  
### Build for development

``cargo make serve``

### Build for production

#### Web

``cargo make build-web``

#### Desktop

``cargo make build-desktop``

### Others

See Makefile.toml at the project root for others commands and details.

## Other tools

### Tests

``cargo test``

### Coverage Test

Install [Tarpauline](https://github.com/xd009642/tarpaulin) with ``cargo install cargo-tarpaulin``

``cargo tarpauline --skip-clean --packages=editor``

## Modules

### editor

Contains the editor application made with iced GUI library.

### vgc

The core of the data to represent a vector graphic.
