const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

// craco does partial overriding of CRA
// Migration example commit: https://gitlab.parity.io/parity/fether/-/commit/a4802f72cebe5db48a6bcebeb1ba5d825426c58c
// Example: https://prestonrichey.com/blog/react-rust-wasm/
// Issue: https://github.com/wasm-tool/wasm-pack-plugin/issues/67

module.exports = {
  plugins: [
    {
      plugin: {
        overrideWebpackConfig: ({ webpackConfig, cracoConfig, pluginOptions, context: { env, paths } }) => {
          const wasmExtensionRegExp = /\.wasm$/;
          webpackConfig.resolve.extensions.push('.wasm');

          webpackConfig.module.rules.forEach(rule => {
            (rule.oneOf || []).forEach(oneOf => {
              if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
                // make file-loader ignore WASM files
                oneOf.exclude.push(wasmExtensionRegExp);
              }
            });
          });

          // Only required when updating rust code within the app
          // 1. point crateDirectory to the rust dir with Cargo.toml
          // 2. the outDir = createDirectory + outDir
          //    - this is the package.json dependency location
          webpackConfig.plugins.push(
            new WasmPackPlugin({
              crateDirectory: path.resolve(__dirname, "victoria-processing"),
              forceMode: 'production',
            })
          );
          return webpackConfig;
        },
      },
      options: {}
    }
  ]
};
