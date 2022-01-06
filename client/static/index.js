import wasm, { log, run_app } from './client.js';

wasm().then(() => {
  window.log = log;
  run_app();
});
