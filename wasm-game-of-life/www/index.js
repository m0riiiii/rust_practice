import {Universe} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const DELAY_RATIO = 8;
let count = 0;
const renderLoop = () => {
    if (count % DELAY_RATIO === 0) {
        pre.textContent = universe.render();
        universe.tick();
    }
    count++;

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);