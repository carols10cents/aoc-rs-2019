import { Screen, Tile, Joystick } from "aoc-13-02";
import { memory } from "aoc-13-02/aoc_13_02_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const WALL_COLOR = "#000000";
const BLOCK_COLOR = "#CC00CC";
const PADDLE_COLOR = "#00CCCC";
const BALL_COLOR = "#CCCC00";

// Construct the game screen, and get its width and height.
let game_screen = Screen.new();
const width = game_screen.width();
const height = game_screen.height();

// Give the canvas room for all of our tiles and a 1px border
// around each of them.
const canvas = document.getElementById("breakout-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');

const game_status = document.getElementById("breakout-status");
let game_started = false;
let game_over = false;
let speed = 10;

const renderLoop = () => {
  setTimeout(function () {
    game_over = game_screen.run();
    game_status.textContent = "Score: " + game_screen.score();

    if (game_over) {
        game_status.textContent += ". Press space to play again";
    } else {
        drawGrid();
        drawCells();
        requestAnimationFrame(renderLoop);
    }
  }, 1000/speed);
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

let joystick = Joystick.Neutral;

const setJoystick = (value) => {
    if (joystick !== value) {
        joystick = value;
        game_screen.set_joystick(value);
    }
}

const keyDownHandler = (event) => {
    if (event.keyCode === 39) { // right
        setJoystick(Joystick.Right);
    } else if (event.keyCode === 37) { // left
        setJoystick(Joystick.Left);
    } else if (event.keyCode === 32 && !game_started) { // space
        game_started = true;
        setJoystick(Joystick.Neutral);
        requestAnimationFrame(renderLoop);
    } else if (event.keyCode === 32 && game_over) {
        game_screen = Screen.new();
        game_started = false;
        game_over = false;

        game_over = game_screen.run();

        drawGrid();
        drawCells();
        game_status.textContent = "Press space to start";

    } else {
        setJoystick(Joystick.Neutral);
    }
};

const keyUpHandler = (event) => {
    setJoystick(Joystick.Neutral);
};

document.addEventListener('keydown', keyDownHandler, false);
document.addEventListener('keyup', keyUpHandler, false);

drawGrid();
drawCells();

game_over = game_screen.run();

drawGrid();
drawCells();
game_status.textContent = "Press space to start";
