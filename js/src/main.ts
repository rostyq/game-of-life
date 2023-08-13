import * as game from "../lib";
import wasm from "../wasm/game_of_life_wasm_bg.wasm?url";

await game.init(wasm);
const cv = document.querySelector<HTMLCanvasElement>("canvas")!;

updateCanvasSize.call(cv);
window.visualViewport?.addEventListener("resize", updateCanvasSize.bind(cv));

game.canvas(cv);
game.run();

function updateCanvasSize(this: HTMLCanvasElement) {
  if (this.width != this.clientWidth || this.height != this.clientHeight) {
    this.width = this.clientWidth;
    this.height = this.clientHeight;
  }
}

