import * as game from "../lib";
import wasm from "../wasm/game_of_life_wasm_bg.wasm?url";

await game.init(wasm);

const canvas = document.querySelector<HTMLCanvasElement>("canvas")!;

updateCanvasSize.call(canvas);
window.visualViewport?.addEventListener("resize", updateCanvasSize.bind(canvas));
canvas.addEventListener("click", toggleCell);

game.canvas(canvas);

document.body.addEventListener("keypress", onkeypress);

function onkeypress(event: KeyboardEvent) {
  switch (event.code) {
    case "Space":
      game.running() ? game.stop() : game.run();
      break;
    case "KeyN":
      game.update();
      break;

    case "KeyC":
      game.seed(false);
      break;

    case "KeyS":
      game.seed(true);
      break;

    case "KeyR":
      game.reset();
      break;
  }

  const num = parseInt(event.key);

  if (Number.isFinite(num) && num !== 0) {
    game.prob(num / 10);
  }
}

function updateCanvasSize(this: HTMLCanvasElement) {
  if (this.width != this.clientWidth || this.height != this.clientHeight) {
    this.width = this.clientWidth;
    this.height = this.clientHeight;
  }
}

function toggleCell(this: HTMLCanvasElement, event: MouseEvent | PointerEvent) {
  // get coordinates on canvas element
  const x = event.clientX - this.offsetLeft;
  const y = event.clientY - this.offsetTop;

  game.toggle(game.index(y), game.index(x));
}

