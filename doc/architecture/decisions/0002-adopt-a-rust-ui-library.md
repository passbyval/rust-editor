# 2. Adopt a Rust UI library

Date: 2024-01-31

## Status

Accepted

## Context

We need to evaluate Rust UI libraries to begin building our user interface.

Must haves

- Accessibility
- Ability to run on macOS, web, and ideally linux.
- Desktop first, web second
- No JS/HTML/CSS on desktop such as electron, however, SSR or WASM may be acceptable
- Fast: little to no UI delay, mostly imperceptible

Nice to haves

- Linux support
- GPU rendering
- Async
- Rust native

Options

- [GPUI](https://github.com/zed-industries/zed/tree/3025e5620d249da498043b125f8bb194c4bee1d2/crates/gpui)
- [Slint](https://github.com/slint-ui/slint)
  - Active development
    - Web
    - Desktop
  - ✅ ❌ Accessibility
    - ✅ Keyboard
    - ❌ Screen reader
- ❌ [conrad](https://github.com/pistondevelopers/conrod)
  - Reactive/Immediate hybrid
  - ❌ Accessibility
- [iced](https://github.com/iced-rs/iced?tab=readme-ov-file)
  - ❌ Experimental
- [tauri](https://tauri.app/)
  - JavaScript/TypeScript driven frontend
- [Dioxus](https://dioxuslabs.com/)
- [cacao](https://docs.rs/cacao/latest/cacao/index.html)
  - Rust bindings for `AppKit`
- ❌ [rui](https://github.com/audulus/rui?tab=readme-ov-file)
  - ❌ Accessibility
- [leptos](https://github.com/leptos-rs/leptos)
  - Reactive/retained
  - Isomorphic
- [egui](https://github.com/emilk/egui)
  - Active development-- high chance of frequent breaking changes
  - ✅ Accessibility
  - Immediate mode
- ❌ [kas](https://github.com/kas-gui/kas)
  - ❌ Accessibility
  - ✅ List virtualization
- GTK 4 ([rust bindings](https://github.com/gtk-rs/gtk4-rs?tab=readme-ov-file))
  - ✅ Accessibility
  - Written in C

## Decision

TBD.

## Consequences

TBD.
