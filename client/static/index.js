import wasm, { log } from './client.js';

wasm().then((module) => {
  window.log = log;
});
