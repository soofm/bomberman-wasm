import { InputType, InputState } from "./InputState";
import { InputOption } from "./InputOption";

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
  inputStates: InputState[] = [];
  private kdEventListener: ((event: KeyboardEvent) => void) | undefined;
  private kuEventListener: ((event: KeyboardEvent) => void) | undefined;
  
  registerInputs(canvas: HTMLCanvasElement, inputOptions: InputOption[]) {
    if (this.kdEventListener) {
      canvas.removeEventListener("keydown", this.kdEventListener);
    }
    if (this.kuEventListener) {
      canvas.removeEventListener("keyup", this.kuEventListener);
    }
    this.inputStates = [];

    const inputMapping = new Map<string, { id: number, key: InputType }>()
    for (let id = 0; id < 4; id++) {
      const inputOption = inputOptions[id]
      if (typeof inputOption !== "undefined") {
        inputMapping.set(inputOption.left, { id, key: InputType.Left })
        inputMapping.set(inputOption.right, { id, key: InputType.Right })
        inputMapping.set(inputOption.up, { id, key: InputType.Up })
        inputMapping.set(inputOption.down, { id, key: InputType.Down })
        inputMapping.set(inputOption.bomb, { id, key: InputType.Bomb })
        this.inputStates.push(new InputState(true))
      } else {
        this.inputStates.push(new InputState(false))
      }
    }

    this.kdEventListener = (event: KeyboardEvent) => {
      const value = inputMapping.get(event.key)
      if (typeof value !== "undefined") {
        this.inputStates[value.id].handleInput(true, value.key);
      }
      event.preventDefault();
    };
    canvas.addEventListener("keydown", this.kdEventListener);
    this.kuEventListener = (event: KeyboardEvent) => {
      const value = inputMapping.get(event.key)
      if (typeof value !== "undefined") {
        this.inputStates[value.id].handleInput(false, value.key);
      }
      event.preventDefault();
    }
    canvas.addEventListener("keyup", this.kuEventListener);
  }

  get inputValues() {
    return new Int32Array(this.inputStates.map(i => i.value))
  }
  
  resetValues() {
    this.inputStates.forEach(i => i.reset())
  }
}