## Snake Game Using Rust

The following are the basic features of the game:

To start the game, press:
* Left, Right, Up, or Down key on your keyboard. 

Once any of the aforementioned key is pressed, the snake will start to move, and they you can still control it with those keys.

Prior to playing the game, be sure to install the following:
* [Qemu](https://www.qemu.org/)
* Nightly Rust:
  * `rustup override set nightly`
* `llvm-tools-preview`:
  * `rustup component add llvm-tools-preview`
* The [bootimage](https://github.com/rust-osdev/bootimage) tool:
  * `cargo install bootimage`