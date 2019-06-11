# bomberman-wasm
This is a Bomberman clone for the web. The game loop is written in Rust, targeting wasm32-unknown-unknown. A small amount of Typescript glue is used to load the assets, handle menus and inputs, and render the game using the PIXI.js library.

This project is purely for fun and educational purposes.

Todo list:

* AI
* Improved menu with support for additional levels and players
* Animations and improved textures

## Build steps
* From the root folder:
* `npm i`
* `npm run build`
* Use `npm run serve` to run `webpack-dev-server` locally
