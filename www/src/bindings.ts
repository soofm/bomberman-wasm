import { GameEngine } from "@victorlee/bomberman-wasm"
import { InputController, PlayerOneInputOptions, PlayerTwoInputOptions } from "./InputController"

const main = <HTMLDivElement>document.getElementById("main")
const initMenu = <HTMLDivElement>document.getElementById("init-menu")
const postMenu = <HTMLDivElement>document.getElementById("post-menu")
const canvas = <HTMLCanvasElement>document.getElementById("canvas")
const ctx = canvas.getContext("2d")
const onePlayer = <HTMLButtonElement>document.getElementById("one-player")
const twoPlayer = <HTMLButtonElement>document.getElementById("two-player")
let animationId: any = null
const inputController = new InputController()
let gameEngine = GameEngine.new();

const renderLoop = () => {
  gameEngine.draw(ctx)
  
  const state = gameEngine.tick(inputController.inputValues)
  inputController.resetValues()
  if (state != 0) {
    stopGame(state)
  }
  else {
    animationId = requestAnimationFrame(renderLoop)
  }
}

const startGame = () => {
  if (initMenu.style.visibility === "visible") {
    initMenu.style.visibility = "hidden"
  } else {
    gameEngine.reset()
  }
  main.style.visibility = "hidden"
  postMenu.style.visibility = "hidden"
  canvas.focus()
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
  inputController.registerInputs(canvas, [PlayerOneInputOptions])
  startGame()
})

twoPlayer.addEventListener("click", (event) => {
  inputController.registerInputs(canvas, [PlayerOneInputOptions, PlayerTwoInputOptions])
  startGame()
})
