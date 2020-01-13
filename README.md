# Paladin

## How to run

To run the game, use

```rust
cargo run --features "vulkan"
```

on Windows and Linux, and

```rust
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```rust
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
