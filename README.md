# bomberman-wasm
This is a Bomberman clone written in Rust, targeting wasm32-unknown-unknown.

It uses the wasm-bindgen library to import the 2D Canvas Javascript API for rendering, and a small amount of HTML and Javascript glue code. This project is purely for fun and educational purposes.

## Build steps
* `wasm-pack build` from the root folder.
* `npm link` from the `/pkg` output folder.
* `npm i; npm link bomberman-wasm` from the `www` folder.
* `npm run serve` to start webpack-dev-server and run the game.