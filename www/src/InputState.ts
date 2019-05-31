export enum InputType {
  Bomb,
  Left,
  Right,
  Up,
  Down,
}

export class InputState {
  state: number = 0;

  constructor(human: boolean) {
    this.state = human? 0 : -1
  }

  handleInput(on: boolean, type: InputType) {
    if (type === InputType.Left) {
      this.setbit(3, on)
      if (on) this.setbit(5, true)
    } else if (type === InputType.Right) {
      this.setbit(2, on)
      if (on) this.setbit(5, true)
    } else if (type === InputType.Up) {
      this.setbit(1, on)
      if (on) this.setbit(5, false)
    } else if (type === InputType.Down) {
      this.setbit(0, on)
      if (on) this.setbit(5, false)
    } else if (type === InputType.Bomb) {
      if (!on) this.setbit(4, true);
    }
  }

  reset() {
    this.setbit(4, false)
  }

  get value(): number {
    return this.state
  }

  private setbit(bit: number, on: boolean) {
    this.state ^= ((on? -1: 0) ^ this.state) & (1 << bit)
  }
}