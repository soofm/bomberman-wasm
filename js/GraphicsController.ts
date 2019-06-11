import * as PIXI from "pixi.js"
import { Tile } from "../crate/pkg"
import * as Constants from "./Constants"
import sand from "./assets/sand.png"
import brick_block from "./assets/brick_block.png"
import steel_block from "./assets/steel_block.png"
import bomb from "./assets/bomb.png"
import explosion from "./assets/explosion.png"
import explosion_ray from "./assets/explosion_ray.png"
import powerup_bomb_number from "./assets/powerup_bomb_number.png"
import powerup_bomb_power from "./assets/powerup_bomb_power.png"
import powerup_speed from "./assets/powerup_speed.png"
import powerup_boot from "./assets/powerup_boot.png"
import player_1 from "./assets/player_1.png"
import player_2 from "./assets/player_2.png"
import player_3 from "./assets/player_3.png"
import player_4 from "./assets/player_4.png"

enum Textures {
  Empty = "sand",
  SoftBlock = "brick_block",
  HardBlock = "steel_block",
  Bomb = "bomb",
  Explosion = "explosion",
  ExplosionRay = "explosion_ray",
  PowerupBombNumber = "powerup_bomb_number",
  PowerupBombPower = "powerup_bomb_power",
  PowerupSpeed = "powerup_speed",
  PowerupBoots = "powerup_boots",
  Player_1 = "player_1",
  Player_2 = "player_2",
  Player_3 = "player_3",
  Player_4 = "player_4",
}

export class GraphicsController {
  app: PIXI.Application
  loader: PIXI.Loader
  gWidth: number = Constants.Width * Constants.TileSize
  gHeight: number = Constants.Height * Constants.TileSize
  tileSprites: ({ sprite: PIXI.Sprite, value: number } | null)[]
  playerSprites: ({ sprite: PIXI.Sprite, value: number })[]
  bombSprites: PIXI.Sprite[]
  explosionContainers: PIXI.Container[]

  constructor(setup: () => void) {
    this.app = new PIXI.Application({width: this.gWidth, height: this.gHeight})
    this.loader = PIXI.Loader.shared
    this.tileSprites = new Array(Constants.Width * Constants.Height)
    this.playerSprites = []
    this.bombSprites = []
    this.explosionContainers = []
    document.getElementById("game")!.appendChild(this.app.view)

    this.loader.add([
      { name: Textures.Empty, url: sand },
      { name: Textures.SoftBlock, url: brick_block },
      { name: Textures.HardBlock, url: steel_block },
      { name: Textures.Bomb, url: bomb },
      { name: Textures.Explosion, url: explosion },
      { name: Textures.ExplosionRay, url: explosion_ray },
      { name: Textures.PowerupBombNumber, url: powerup_bomb_number },
      { name: Textures.PowerupBombPower, url: powerup_bomb_power },
      { name: Textures.PowerupSpeed, url: powerup_speed },
      { name: Textures.PowerupBoots, url: powerup_boot },
      { name: Textures.Player_1, url: player_1 },
      { name: Textures.Player_2, url: player_2 },
      { name: Textures.Player_3, url: player_3 },
      { name: Textures.Player_4, url: player_4 },
    ]).load(setup)
  }

  public init = (tiles: Uint8Array) => {
    const empty = new PIXI.TilingSprite(this.loader.resources[Textures.Empty].texture, this.gWidth, this.gHeight)
    this.app.stage.addChild(empty)

    for (let row = 0; row < Constants.Height; row += 1) {
      for (let col = 0; col < Constants.Width; col += 1) {
        const tile = tiles[row * Constants.Width + col]
        const texture = this.getTileTexture(tile)
        if (texture) {
          const sprite = this.getSprite(texture, col, row, 0)
          this.app.stage.addChild(sprite)
          this.tileSprites[row * Constants.Width + col] = { sprite, value: tile }
        }
      }
    }

    for (let i = 0; i < Constants.PlayerCount; i++) {
      const texture = this.getPlayerTexture(i, true)
      if (texture) {
        const sprite = this.getSprite(texture, 0, 0, 0)
        this.app.stage.addChild(sprite)
        this.playerSprites[i] = { sprite, value: 1 }
      }
    }
  }

  public renderTiles = (tiles: Uint8Array) => {
    for (let row = 0; row < Constants.Height; row += 1) {
      for (let col = 0; col < Constants.Width; col += 1) {
        const i = row * Constants.Width + col
        const tile = tiles[i]
        if (this.tileSprites[i]) {
          if (tile === Tile.Empty) {
            this.app.stage.removeChild(this.tileSprites[i]!.sprite)
            this.tileSprites[i] = null
          } else if (tile !== this.tileSprites[i]!.value) {
            const texture = this.getTileTexture(tile)
            if (texture) {
              this.tileSprites[i]!.sprite.texture = texture
              this.tileSprites[i]!.value = tile
            }
          }
        }
      }
    }
  }

  public renderPlayer = (data: number, x: number, y: number) => {
    const i = data & 255
    const isAlive = (data >> 8) & 1
    this.playerSprites[i].sprite.x = x * Constants.TileSize
    this.playerSprites[i].sprite.y = y * Constants.TileSize
    if (!isAlive) {
      this.getPlayerTexture(i, false)
      this.playerSprites[i].sprite.texture
    }
  }

  public addBomb = (x: number, y: number) => {
    const texture = this.loader.resources[Textures.Bomb].texture
    const sprite = this.getSprite(texture, x, y, 0)
    this.app.stage.addChild(sprite)
    this.bombSprites.push(sprite)
  }

  public moveBomb = (index: number, x: number, y: number) => {
    this.bombSprites[index].x = x * Constants.TileSize
    this.bombSprites[index].y = y * Constants.TileSize
  }

  public removeBomb = () => {
    const bomb = this.bombSprites.shift()
    if (bomb) { this.app.stage.removeChild(bomb) }
  }

  public addExplosion = (x: number, y: number, explosionSize: number) => {
    console.log(JSON.stringify({x, y, explosionSize}))
    const left = explosionSize >> 24
    const right = (explosionSize >> 16) & 255
    const up = (explosionSize >> 8) & 255
    const down = explosionSize & 255

    let container = new PIXI.Container()
    const explosionTexture = this.loader.resources[Textures.Explosion].texture
    const explosionRayTexture = this.loader.resources[Textures.ExplosionRay].texture
    const sprite = this.getSprite(explosionTexture, 0, 0, 0)
    container.addChild(sprite)
    console.log(`left=${left}, right=${right}, up=${up}, down=${down}`)
    for (let i = 1; i <= left; i++) {
      container.addChild(this.getSprite(explosionRayTexture, -i, 0, 0))
    }
    for (let i = 1; i <= right; i++) {
      container.addChild(this.getSprite(explosionRayTexture, i, 0, 0))
    }
    for (let i = 1; i <= up; i++) {
      container.addChild(this.getSprite(explosionRayTexture, 1, -i, Math.PI / 2))
    }
    for (let i = 1; i <= down; i++) {
      container.addChild(this.getSprite(explosionRayTexture, 1, i, Math.PI / 2))
    }
    container.x = x * Constants.TileSize
    container.y = y * Constants.TileSize
    this.app.stage.addChild(container)
    this.explosionContainers.push(container)
  }

  public removeExplosion = () => {
    const explosion = this.explosionContainers.shift()
    if (explosion) { this.app.stage.removeChild(explosion) }
  }

  private getSprite = (texture: PIXI.Texture, x: number, y: number, rotation: number) => {
    const sprite = new PIXI.Sprite(texture)
    sprite.x = x * Constants.TileSize
    sprite.y = y * Constants.TileSize
    sprite.width = Constants.TileSize
    sprite.height = Constants.TileSize
    sprite.rotation = rotation
    return sprite
  }

  private getTileTexture = (tile: number): PIXI.Texture | undefined => {
    let texture
    switch (tile) {
      case Tile.Empty:
        break
      case Tile.HardBlock:
        texture = this.loader.resources[Textures.HardBlock].texture
        break
      case Tile.SoftBlock:
        texture = this.loader.resources[Textures.SoftBlock].texture
        break
      case Tile.Bomb:
        break
      case Tile.PowerupBombNumber:
        texture = this.loader.resources[Textures.PowerupBombNumber].texture
        break
      case Tile.PowerupBombPower:
        texture = this.loader.resources[Textures.PowerupBombPower].texture
        break
      case Tile.PowerupSpeed:
        texture = this.loader.resources[Textures.PowerupSpeed].texture
        break
      case Tile.PowerupBoots:
        texture = this.loader.resources[Textures.PowerupBoots].texture
        break
      default:
        throw new Error(`Could not find texture: ${tile}`)
    }
    return texture
  }

  private getPlayerTexture = (id: number, isAlive: boolean): PIXI.Texture | undefined => {
    let texture
    switch (id) {
      case 0:
        texture = this.loader.resources[Textures.Player_1].texture
        break
      case 1:
        texture = this.loader.resources[Textures.Player_2].texture
        break
      case 2:
        texture = this.loader.resources[Textures.Player_3].texture
        break
      case 3:
        texture = this.loader.resources[Textures.Player_4].texture
        break
      default:
        throw new Error(`Could not find player texture: ${id}`)
    }
    if (isAlive) {
      texture.frame = new PIXI.Rectangle(0, 0, Constants.SpriteSize, Constants.SpriteSize)
    } else {
      texture.frame = new PIXI.Rectangle(Constants.SpriteSize, 0, Constants.SpriteSize, Constants.SpriteSize)
    }
    return texture
  }
}