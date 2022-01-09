import * as wasm from "candle-chart";

let lastFrame;
const run = (ts) => {
    if (lastFrame == null) {
        lastFrame = ts;
    }

    if (ts - lastFrame > 200) {
        wasm.run();
        lastFrame = ts;
    }

    window.requestAnimationFrame(run);
};

window.requestAnimationFrame(run);
