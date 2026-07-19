# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Pixxxel is a small Rust desktop application for creating pixel-art game assets. It opens a window (via `minifb`) showing a scaled-up pixel canvas that the user paints on with the mouse.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Run a single test: `cargo test test_name` (e.g. `cargo test set_out_of_bounds_returns_error`)
- Check without building: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Architecture

The crate is split into a library (`src/lib.rs`) and a binary (`src/main.rs`):

- **`src/lib.rs`** — defines `Canvas<T>`, a generic 2D grid backed by a flat `Vec<T>` (row-major indexing: `index = x + y * width`). Bounds-checked reads/writes go through `get`/`set`, which return `Option`/`Result<(), CanvasError>` rather than panicking on out-of-range coordinates. `Canvas<u32>` additionally implements `render_into`, which upscales each logical cell into a `scale x scale` block of pixels in a target framebuffer — this is what bridges the logical pixel-art grid to the window's actual pixel buffer.
- **`src/main.rs`** — owns the `minifb` window/event loop. Each frame it: renders the canvas into the window's pixel buffer via `Canvas::render_into`, reads the mouse position and translates window coordinates into canvas coordinates by dividing out `SCALE`, and applies left-click (draw) / right-click (erase) as `Canvas::set` calls. Canvas size, scale factor, and window dimensions are defined as constants at the top of the file (`CANVAS_SIZE`, `SCALE`, `WIDTH`, `HEIGHT`).

The `Canvas` struct also carries an 8-color `palette` and `primary_color`/`secondary_color` fields intended for a color-picker UI; as of now these are initialized but not yet wired into the render or input loop in `main.rs`.
