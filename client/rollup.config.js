import livereload from 'rollup-plugin-livereload';

import rust from '@wasm-tool/rollup-plugin-rust';

const hot = !!process.env.ROLLUP_WATCH;

const plugins = [
    rust({
      verbose: true,
      serverPath: '/public/',
    }),
];

if (hot) {
    plugins.push(livereload({
        watch: '../static',
        delay: 300,
    }))
}

export default {
    input: 'index.js',
    output: {
        dir: '../static',
        format: 'iife',
        sourcemap: hot,
    },
    treeshake: !hot,
    plugins,
}