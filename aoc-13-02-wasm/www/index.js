import { Computer } from "aoc-13-02";

const pre = document.getElementById("breakout-canvas");
const computer = Computer.new();

const renderLoop = () => {
  pre.textContent = computer.render();
  computer.run();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
