import { Screen, Tile } from "aoc-13-02";
import { memory } from "aoc-13-02/aoc_13_02_bg";

const CELL_SIZE = 12; // px
const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const WALL_COLOR = "#000000";
const BLOCK_COLOR = "#CC00CC";
const PADDLE_COLOR = "#00CCCC";
const BALL_COLOR = "#CCCC00";

// Construct the game screen, and get its width and height.
const game_screen = Screen.new();
const width = game_screen.width();
const height = game_screen.height();

// Give the canvas room for all of our tiles and a 1px border
// around each of them.
const canvas = document.getElementById("breakout-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  game_screen.run();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
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
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const tilesPtr = game_screen.tiles();
  const tiles = new Uint8Array(memory.buffer, tilesPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      const tile = tiles[idx];

      if (tile === Tile.Wall) {
          ctx.fillStyle = WALL_COLOR;
      } else if (tile === Tile.Block) {
          ctx.fillStyle = BLOCK_COLOR;
      } else if (tile === Tile.Paddle) {
          ctx.fillStyle = PADDLE_COLOR;
      } else if (tile === Tile.Ball) {
          ctx.fillStyle = BALL_COLOR;
      } else {
          ctx.fillStyle = EMPTY_COLOR;
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
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
