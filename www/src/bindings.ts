import { GameEngine } from "@victorlee/bomberman-wasm"
import { InputController, PlayerOneInputOptions, PlayerTwoInputOptions } from "./InputController"

const main = document.getElementById("main")
const canvas = <HTMLCanvasElement>document.getElementById("canvas")
const ctx = canvas.getContext("2d")
const onePlayer = document.getElementById("one-player")
const twoPlayer = document.getElementById("two-player")
let animationId: any = null
const inputController = new InputController()
const gameEngine = GameEngine.new();

const renderLoop = () => {
  gameEngine.draw(ctx)
  gameEngine.tick(inputController.inputValues)
  inputController.resetValues()

  animationId = requestAnimationFrame(renderLoop)
}

const startGame = () => {
  main!.style.visibility = "hidden"
  canvas.focus();
  renderLoop()
}

const stopGame = () => {
  cancelAnimationFrame(animationId)
  animationId = null
}

onePlayer!.addEventListener("click", (event) => {
  inputController.registerInputs(canvas, [PlayerOneInputOptions])
  startGame()
})

twoPlayer!.addEventListener("click", (event) => {
  inputController.registerInputs(canvas, [PlayerOneInputOptions, PlayerTwoInputOptions])
  startGame()
})
