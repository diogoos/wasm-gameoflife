# Game of Life
This is a web-based implementation of Conway's Game of Life written in Rust and WebAssembly. The Game of Life is a cellular automaton that simulates the evolution of cells based on a set of simple rules.

## Features
* Click to draw: You can interact with the simulation by clicking on the cells to toggle them between alive and dead states.
* Pause and play: You can pause or resume the simulation at any time to observe or modify the state of the cells.
* Step through generations: You can step through the generations of the simulation manually by clicking the "Step" button. This allows you to observe the progression of the cells one generation at a time.
* Adjust background and framerate: You have the ability to adjust the background color and control the framerate of the simulation to speed up or slow down the animation.

## Building from source
To run the Game of Life locally, you will need to have Rust and Node.js installed on your machine.

1. Clone this repository
```bash
git clone https://github.com/diogoos/wasm-gameoflife.git
```
2. Install node & webpack dependencies
```bash
npm install
```
3. Install cargo dependencies by building from source
```bash
cargo build
```
4. Run the app
```bash
npm run serve
```
5. Access locally from your browser at localhost:8080

