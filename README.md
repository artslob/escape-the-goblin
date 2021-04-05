# Escape the goblin

## About

Simple game written in Rust with [Tetra](https://github.com/17cupsofcoffee/tetra) framework.

I got idea for this game from this [video](https://youtu.be/V0V3LMK40iI). Game story:
> You find yourself in the middle of circular lake. There is a goblin at the shore who will eat you
> immediately if he can catch you. Luckily the goblin can`t swim. You know if you make it to land
> you can sprint off to the forest and escape. The problem is you are slow in water: the goblin is
> 4 time faster. The goblin always moves optimally which means moving to the spot closest to you.
> Can you escape the goblin?

The point is you can`t just swim to the shore in straight line in opposite to goblin direction,
because you need to move radius of circle, goblin needs to move 3.14 * radius, but goblin
moves 4 times faster.

The solution is to swim in circle around center of the lake so goblin always need to catch up.
The best circle radius is ratio of speeds: lake\`s radius / 4.

## Developing

Installing dependencies on Linux Ubuntu:
```bash
sudo apt install libsdl2-dev
sudo apt install libasound2-dev
```
Running:
```bash
cargo run --release
```
