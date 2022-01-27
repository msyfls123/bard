import wasm from './Cargo.toml';

wasm().then(({ log, run_app }) => {
  window.log = log;
  run_app();
});
