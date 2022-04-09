import livereload from 'rollup-plugin-livereload';
import styles from 'rollup-plugin-styles';

import rust from '@wasm-tool/rollup-plugin-rust';

const hot = !!process.env.ROLLUP_WATCH;

const plugins = [
    rust({
      verbose: true,
      serverPath: '/public/',
    }),
    styles(),
];

if (hot) {
    plugins.push(livereload({
        watch: '../static',
        delay: 300,
    }))
}

export default {
    input: 'src/index.js',
    output: {
        dir: '../static',
        format: 'iife',
        assetFileNames: '[name]-[hash][extname]',
        sourcemap: hot,
    },
    treeshake: !hot,
    plugins,
}