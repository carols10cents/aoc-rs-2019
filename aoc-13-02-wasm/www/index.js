import { Screen } from "aoc-13-02";

const pre = document.getElementById("breakout-canvas");
const screen = Screen.new();

const renderLoop = () => {
  pre.textContent = screen.render();
  screen.run();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
