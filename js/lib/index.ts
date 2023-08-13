import { type InitInput, default as _init, World } from "../wasm/game_of_life_wasm";

let world: World | null = null;
let cv: HTMLCanvasElement | null = null;
let ctx: CanvasRenderingContext2D | null = null;
let memory: WebAssembly.Memory;

let canvasWidth: number;
let canvasHeight: number;

let colorMap = ["white", "black"];
let borderSize = 1;
let cellSize = 10;
let delay = 150;
let prob = 0.5;

let timeoutId: number;

export async function init(input: InitInput) {
  const output = await _init(input);
  memory = output.memory;
}

export function canvas(element: HTMLCanvasElement | null) {
  if (element?.isSameNode(cv)) {
    return;
  }

  world && world.free();
  world = null, cv = null, ctx = null;

  if (element !== null) {
    cv = element;
    ctx = cv.getContext("2d");
    createWorld(cv);
  }
}

export function run() {
  if (!world || !cv) {
    return clearTimeout(timeoutId);
  }

  if (canvasWidth == cv.width && canvasHeight == cv.height) {
    world.update();
  } else {
    createWorld(cv);
  }

  requestAnimationFrame(() => drawCells(world!, ctx!));

  timeoutId = setTimeout(run, delay);
}

export function stop() {
  clearTimeout(timeoutId);
}

function createWorld(canvas: HTMLCanvasElement) {
  world && world.free();

  canvasWidth = canvas.width, canvasHeight = canvas.height;

  const cell = cellSize + borderSize;

  const width = (canvasWidth) / cell;
  const height = (canvasHeight) / cell;

  const seed = generateSeed();
  const t0 = performance.now();
  world = World.random(width, height, seed, prob);
  const time = performance.now() - t0;
  console.info(`[game-of-life] created world ${world.width}x${world.height} in ${time} ms with seed ${seed}`);
}

function drawCells(world: World, ctx: CanvasRenderingContext2D) {
  const cells = new Uint8Array(memory.buffer, world.pointer, world.size);
  ctx.beginPath();

  for (let row = 0; row < world.height; row++) {
    for (let col = 0; col < world.width; col++) {
      const idx = row * world.width + col;

      ctx.fillStyle = colorMap[cells[idx]];

      ctx.fillRect(
        col * (cellSize + borderSize) + borderSize,
        row * (cellSize + borderSize) + borderSize,
        cellSize,
        cellSize
      );
    }
  }

  ctx.stroke();
}

function generateSeed() {
  const array = new Float64Array([Math.random()]);
  const view = new DataView(array.buffer);
  return view.getBigUint64(0);
}
