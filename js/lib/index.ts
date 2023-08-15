import { type InitInput, default as _init, World } from "../wasm/game_of_life_wasm";

interface Config {
  seed: bigint | null,
  prob: number,
  border: number,
  cell: number,
  color: string,
}

const NAME = "game-of-life";
const LOG_PREFIX = `[${NAME}]`;

const DEFAULT_BORDER = 1;
const DEFAULT_CELL = 10;
const DEFAULT_PROB = 0.5;
const DEFAULT_COLOR = "black";

let _border = DEFAULT_BORDER;
let _cell = DEFAULT_CELL;
let _prob = DEFAULT_PROB;
let _color = DEFAULT_COLOR;
let _seed: bigint | null = null;

let _step = _cell + _border;

let world: World | null = null;
let cv: HTMLCanvasElement | null = null;
let ctx: CanvasRenderingContext2D | null = null;
let memory: WebAssembly.Memory | null;

let _width: number;
let _height: number;

let runId: number | null = null;

export async function init(input: InitInput) {
  memory = (await _init(input)).memory;
}

export function config(c: Partial<Config> = {}): Config {
  seed(c.seed);
  prob(c.prob);
  c.border
    ? _border = c.border
    : c.border === null && (_border = DEFAULT_BORDER);
  c.cell
    ? _cell = c.cell
    : c.cell === null && (_cell = DEFAULT_CELL);

  (c.cell || c.border) && (_step = _cell + _border);

  return { seed: _seed, prob: _prob, border: _border, cell: _cell, color: _color };
}

export function canvas(element?: HTMLCanvasElement | null): HTMLCanvasElement | null {
  if (!memory) throw new Error("game-of-life need to be initialized");
  if (element === undefined || element?.isSameNode(cv)) return cv;
  if (element && !element.getContext) throw new Error("element is not a canvas");

  world?.free(), world = null, cv = null, ctx = null;

  if (element !== null) {
    _width = element.width, _height = element.height;
    cv = element, ctx = cv.getContext("2d"), _create();
  }

  return cv;
}

export function running(): boolean {
  return Boolean(runId);
}

export function update() {
  if (!runId) _update();
}

export function run() {
  if (!runId) _run();
}

export function index(value: number) {
  return Math.round((value - value % _step) / _step);
}

export function toggle(row: number, column: number) {
  world?.toggle(row, column);
  if (!runId) _draw();
}

export function stop() {
  runId && cancelAnimationFrame(runId), runId = null;
}

export function reset() {
  if (cv) {
    _create();
    _draw();
  }
}

export function seed(value?: bigint | null | boolean): bigint | null {
  switch (typeof value) {
    case "bigint":
      _seed = value;
      break;
    case "boolean":
      _seed = value ? generateSeed() : null;
      break;
  }
  
  reset();
  return _seed;
}

export function prob(value?: number) {
  _prob = value !== undefined ? Math.max(0, Math.min(value)) : _prob;

  if (_seed !== null) {
    reset();
  }
  return _prob;
}

function _run() {
  if (!world || !cv || !ctx) return stop();
  _update();
  runId = requestAnimationFrame(_run);
}

function _update() {
  if (_width == cv!.width && _height == cv!.height) {
    world!.update();
  } else {
    console.info(LOG_PREFIX, "canvas resized");
    _width = cv!.width, _height = cv!.height;
    _create();
  }

  _draw();
}

function _create() {
  world?.free();

  const w = _width / _step;
  const h = _height / _step
 
  if (_seed !== null) {
    world = World.random(w, h, _seed, _prob);
  } else {
    world = new World(w, h);
  }

  console.info(
    LOG_PREFIX, `create world`, "\n\t",
    ...(_seed !== null ? [
    "seed:", _seed.toString(), "\n\t",
    "probability:", Math.round(_prob * 100), "%", "\n\t",
    ] : []),
    "size:", world.width, "x", world.height, "\n\t",
    "cells:", world.size.toLocaleString(), "\n\t",
    "population:", world.population.toLocaleString(),
  );
}

function _draw() {
  ctx!.clearRect(0, 0, _width, _height);
  const view = new DataView(memory!.buffer, world!.pointer, world!.size);

  ctx!.fillStyle = _color;

  for (let row = 0; row < world!.height; row++) {
    for (let column = 0; column < world!.width; column++) {
      const offset = row * world!.width + column;

      const y = row * (_cell + _border) + _border;
      const x = column * (_cell + _border) + _border;

      if (view.getUint8(offset) === 1) {
        ctx!.fillRect(x, y, _cell, _cell);
      }
    }
  }
}

function generateSeed() {
  const array = crypto.getRandomValues(new Uint8Array(8));
  const view = new DataView(array.buffer);
  return view.getBigUint64(0);
}
