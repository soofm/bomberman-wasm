import { GameEngine } from "../crate/pkg"
import { memory } from "../crate/pkg/bomberman_wasm_bg"
import * as Constants from "./Constants"
import { InputController, PlayerOneInputOptions, PlayerTwoInputOptions } from "./InputController"
import { GraphicsController } from "./GraphicsController"

const main = <HTMLDivElement>document.getElementById("main")
const postMenu = <HTMLDivElement>document.getElementById("post-menu")
const onePlayer = <HTMLButtonElement>document.getElementById("one-player")
const twoPlayer = <HTMLButtonElement>document.getElementById("two-player")
let animationId: any = null
let gameEngine: GameEngine
let loaded = false

const sendInput = (playerId: number, inputType: number, on: boolean): void => {
  gameEngine.send_input(playerId, inputType, on)
}

const setup = () => {
  loaded = true
}

const inputController = new InputController(sendInput)
const graphicsController = new GraphicsController(setup)

const renderLoop = () => {
  const state = gameEngine.tick()
  
  const tilesPtr = gameEngine.get_tiles_ptr()
  const tiles = new Uint8Array(memory.buffer, tilesPtr, Constants.Width * Constants.Height)
  graphicsController.renderTiles(tiles)

  gameEngine.render_players(graphicsController.renderPlayer)
  gameEngine.render_bombs(graphicsController.addBomb, graphicsController.moveBomb, graphicsController.removeBomb)
  gameEngine.render_explosions(graphicsController.addExplosion, graphicsController.removeExplosion)
  
  if (state != 0) {
    inputController.deregisterInputs()
    stopGame(state)
  } else {
    animationId = requestAnimationFrame(renderLoop)
  }
}

const startGame = (players: number) => {
  gameEngine = GameEngine.new()
  if (players > 0) { gameEngine.set_human_player(0) }
  if (players > 1) { gameEngine.set_human_player(1) }
  if (players > 2) { gameEngine.set_human_player(2) }
  if (players > 3) { gameEngine.set_human_player(3) }
  const tilesPtr = gameEngine.get_tiles_ptr()
  const tiles = new Uint8Array(memory.buffer, tilesPtr, Constants.Width * Constants.Height)
  graphicsController.init(tiles)
  main.classList.remove("init", "post")
  renderLoop()
}

const stopGame = (state: number) => {
  main.classList.add("post")
  if (state > 0) {
    postMenu.firstElementChild!.textContent = `Player ${state} wins!`
  } else {
    postMenu.firstElementChild!.textContent = "TIE!"
  }

  cancelAnimationFrame(animationId)
  animationId = null
  gameEngine.free()
  graphicsController.reset()
}

onePlayer.addEventListener("click", (event) => {
  if (!loaded) { return }
  inputController.registerInputs([PlayerOneInputOptions])
  startGame(1)
})

twoPlayer.addEventListener("click", (event) => {
  if (!loaded) { return }
  inputController.registerInputs([PlayerOneInputOptions, PlayerTwoInputOptions])
  startGame(2)
})
