import { InputOption } from "./InputOption";
import { InputType } from "../crate/pkg";

export const PlayerOneInputOptions: InputOption = {
  left: "a",
  right: "d",
  up: "w",
  down: "s",
  bomb: "e"
}

export const PlayerTwoInputOptions: InputOption = {
  left: "ArrowLeft",
  right: "ArrowRight",
  up: "ArrowUp",
  down: "ArrowDown",
  bomb: "Enter"
}

export class InputController {
  private kdEventListener: ((event: KeyboardEvent) => void) | undefined;
  private kuEventListener: ((event: KeyboardEvent) => void) | undefined;

  constructor(private sendInputCallback: (id: number, inputType: InputType, on: boolean) => void) {
  }
  
  registerInputs(inputOptions: InputOption[]) {
    if (this.kdEventListener) {
      window.removeEventListener("keydown", this.kdEventListener)
    }
    if (this.kuEventListener) {
      window.removeEventListener("keyup", this.kuEventListener)
    }

    const inputMapping = new Map<string, { id: number, key: InputType }>()
    inputOptions.forEach((inputOption, id) => {
      inputMapping.set(inputOption.left, { id, key: InputType.Left })
      inputMapping.set(inputOption.right, { id, key: InputType.Right })
      inputMapping.set(inputOption.up, { id, key: InputType.Up })
      inputMapping.set(inputOption.down, { id, key: InputType.Down })
      inputMapping.set(inputOption.bomb, { id, key: InputType.Bomb })
    })

    this.kdEventListener = (event: KeyboardEvent) => {
      const value = inputMapping.get(event.key)
      if (typeof value !== "undefined") {
        this.sendInputCallback(value.id, value.key, true)
        event.preventDefault()
      }
    };
    window.addEventListener("keydown", this.kdEventListener)
    this.kuEventListener = (event: KeyboardEvent) => {
      const value = inputMapping.get(event.key)
      if (typeof value !== "undefined") {
        this.sendInputCallback(value.id, value.key, false)
        event.preventDefault()
      }
    }
    window.addEventListener("keyup", this.kuEventListener)
  }
}