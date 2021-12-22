rm -rf static
wasm-pack build client --target web --out-name client --out-dir ../static
cp -R client/static/ static/
