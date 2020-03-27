# Paladin

Paladin is a player vs. player or player vs. CPU duel in the vein of Star Control. It's also a game I'm building to teach myself gamedev and Rust, so it is pretty raw.

## Controls

### Green Ship

* 'W' == Thrust
* 'A' == Rotate Counterclockwise
* 'D' == Rotate Clockwise
* 'SPACE' == Shoot Laser

### Red Ship

* 'Up' == Thrust
* 'Left' == Rotate Counterclockwise
* 'Right' == Rotate Clockwise
* 'Right CTRL' == Shoot Laser

## Game Physics

Paladin has newtonian physics and momentum with forward thrust being your primary means of movement. If you want to slow down, turn around and apply opposite thrust.

The game map warps around, so you can both shoot and travel through the sides of the map.

Each player starts the game with 150 hit points. Attacks from lasers, missiles (eventually) or collisions reduce this total.

Laser strikes also impart momentum and apply slight jitter to the ship struck.

When a ship is reduced to 0 or fewer hit points, it is destroyed. The surviving player gains a point and both ships are restored and returned to their starting positions.

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

