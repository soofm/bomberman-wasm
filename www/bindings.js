import { GameData, Tile } from "bomberman-wasm";
import { memory } from "bomberman-wasm/bomberman_wasm_bg";

const CELL_SIZE = 50;
const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const SOFT_BLOCK_COLOR = "#D2B48C";
const HARD_BLOCK_COLOR = "#000000";
const world = GameData.new();
const width = world.width();
const height = world.height();

const canvas = document.getElementById("game-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    //world.tick();

    drawGrid();
    drawCells();
    
    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const drawCells = () => {
    const tilesPtr = world.tiles();
    const tiles = new Uint8Array(memory.buffer, tilesPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        if (tiles[idx] === Tile.Empty) {
            ctx.fillStyle = EMPTY_COLOR;
        } else if (tiles[idx] === Tile.SoftBlock) {
            ctx.fillStyle = SOFT_BLOCK_COLOR;
        } else {
            ctx.fillStyle = HARD_BLOCK_COLOR;
        }

        ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        );
        }
    }

    ctx.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
};