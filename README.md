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

## Resources

This game is a learning project. The resources I've used to piece it together include:

* [Amethyst Book](https://book.amethyst.rs/stable/) -- basics of using Amethyst
* [Pong Tutorial](https://book.amethyst.rs/stable/pong-tutorial.html) -- basic walkthrough of building a game in Amethyst
* [Space Menace](https://github.com/amethyst/space-menace) - animation + ECS formatting for an Amethyst game
* [Open Game Art](https://opengameart.org) -- great resources in art, music and sound for games
* [Asteroids-Amethyst](https://github.com/udoprog/asteroids-amethyst) -- modelling lasers and thrust
* [Space Shooter](https://github.com/amethyst/space_shooter_rs) -- looking into this now

