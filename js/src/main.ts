import * as game from "../lib";
import wasm from "../wasm/game_of_life_wasm_bg.wasm?url";

App(document.body);

export default async function App(root: Node) {
  await game.init(wasm);
  game.seed(true);

  const canvas = document.createElement("canvas")!;
  root.appendChild(canvas);
  updateCanvasSize.call(canvas);

  game.canvas(canvas);

  canvas.addEventListener("click", toggleCell);

  window.visualViewport?.addEventListener("resize", updateCanvasSize.bind(canvas));
  document.body.addEventListener("keypress", onkeypress);

  game.run();

  function onkeypress(event: KeyboardEvent) {
    switch (event.code) {
      case "Space":
        game.running() ? game.stop() : game.run();
        break;

      case "KeyN":
        game.update();
        break;

      case "KeyB":
        game.reset();
        break;

      case "KeyR":
        game.seed(true);
        break;

      case "KeyE":
        game.seed(false);
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
    if (game.running()) return;

    // get coordinates on canvas element
    const x = game.index(event.clientX - this.offsetLeft);
    const y = game.index(event.clientY - this.offsetTop);

    game.toggle(y, x);
  }
}
