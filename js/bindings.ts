import { GameEngine } from "../crate/pkg"
import { memory } from "../crate/pkg/bomberman_wasm_bg"
import * as Constants from "./Constants"
import { InputController, PlayerOneInputOptions, PlayerTwoInputOptions } from "./InputController"
import { GraphicsController } from "./GraphicsController";

const main = <HTMLDivElement>document.getElementById("main")
const initMenu = <HTMLDivElement>document.getElementById("init-menu")
const postMenu = <HTMLDivElement>document.getElementById("post-menu")
const onePlayer = <HTMLButtonElement>document.getElementById("one-player")
const twoPlayer = <HTMLButtonElement>document.getElementById("two-player")
let animationId: any = null
let gameEngine = GameEngine.new()
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
  const tilesPtr = gameEngine.get_tiles_ptr()
  const tiles = new Uint8Array(memory.buffer, tilesPtr, Constants.Width * Constants.Height)
  graphicsController.renderTiles(tiles)

  gameEngine.render_players(graphicsController.renderPlayer)
  gameEngine.render_bombs(graphicsController.addBomb, graphicsController.moveBomb, graphicsController.removeBomb)
  gameEngine.render_explosions(graphicsController.addExplosion, graphicsController.removeExplosion)
  
  const state = gameEngine.tick()
  if (state != 0) {
    stopGame(state)
  }
  else {
    animationId = requestAnimationFrame(renderLoop)
  }
}

const startGame = () => {
  const tilesPtr = gameEngine.get_tiles_ptr()
  const tiles = new Uint8Array(memory.buffer, tilesPtr, Constants.Width * Constants.Height)
  graphicsController.init(tiles)
  if (initMenu.style.visibility !== "hidden") {
    initMenu.style.visibility = "hidden"
  } else {
    gameEngine.reset()
  }
  main.style.visibility = "hidden"
  postMenu.style.visibility = "hidden"
  renderLoop()
}

const stopGame = (state: number) => {
  main.style.visibility = "visible"
  postMenu.style.visibility = "visible"
  if (state > 0) {
    postMenu.firstElementChild!.textContent = `Player ${state} wins!`
  } else {
    postMenu.firstElementChild!.textContent = "TIE!"
  }

  cancelAnimationFrame(animationId)
  animationId = null
}

onePlayer.addEventListener("click", (event) => {
  if (!loaded) { return }
  gameEngine.set_human_player(0)
  inputController.registerInputs([PlayerOneInputOptions])
  startGame()
})

twoPlayer.addEventListener("click", (event) => {
  if (!loaded) { return }
  gameEngine.set_human_player(0)
  gameEngine.set_human_player(1)
  inputController.registerInputs([PlayerOneInputOptions, PlayerTwoInputOptions])
  startGame()
})
