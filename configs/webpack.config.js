const path = require("path");
const dist = path.resolve(__dirname, "../dist");

const WebpackBar = require("webpackbar");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = (env, argv) => {
  return {
    entry: {
      // bundle root with name app.js
      app: path.resolve(__dirname, "../entries/index.ts")
    },
    output: {
      // TravicCI or you can deploy your site from this folder (after `yarn build:release`)
      path: dist
    },
    devServer: {
      contentBase: dist,
      hot: true,
      // you can connect to dev server from devices in your network (e.g. 192.168.0.3:3000)
      host: "0.0.0.0",
      port: 8000,
      noInfo: true,
      stats: "errors-only",
      // Note: it doesn't work for Rust files (probably because Rust isn't compiled by loader but plugin)
      overlay: {
        warnings: true,
        errors: true
      }
    },
    plugins: [
      // show compilation progress bar in console
      new WebpackBar(),
      // clean dist folder before compilation
      new CleanWebpackPlugin(),
      // add scripts, css, ... to html template
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "../entries/index.html"),
        hash: true
      }),
      // compile Rust
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../crate"),
        outDir: path.resolve(__dirname, "../crate/pkg")
      }),

      // Uncomment when you have problems with Edge (= when small polyfill in index.html doesn't work)
      //
      // Have this example work in Edge which doesn't ship `TextEncoder` or
      // `TextDecoder` at this time.
      // new webpack.ProvidePlugin({
      //   TextDecoder: ['text-encoding', 'TextDecoder'],
      //   TextEncoder: ['text-encoding', 'TextEncoder']
      // }),

      // you can find files from folder ../static on url http://my-site.com/static/
      new CopyWebpackPlugin([
        {
          from: "static",
          to: "static"
        }
      ])
    ],
    // webpack try to guess how to resolve imports in this order:
    resolve: {
      extensions: [".ts", ".js", ".wasm"],
      alias: {
        crate: path.resolve(__dirname, "../crate")
      }
    },
    module: {
      rules: [
        {
          test: /\.(png|jpg|gif)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                // don't copy files to dist, we do it through CopyWebpackPlugin (see above)
                // - we only want to resolve urls to these files
                emitFile: false,
                name: "[path][name].[ext]"
              }
            }
          ]
        },
        {
          test: /\.ts$/,
          loader: "ts-loader?configFile=configs/tsconfig.json"
        },
        {
          test: /\.css$/,
          use: [
            "style-loader",
            {
              loader: "css-loader",
              options: {
                // https://github.com/webpack-contrib/css-loader/issues/228
                importLoaders: 1
              }
            },
            {
              loader: "postcss-loader",
              options: {
                config: {
                  // path to postcss.config.js
                  path: __dirname,
                  // send mode into postcss.config.js (see more info in that file)
                  ctx: { mode: argv.mode }
                }
              }
            }
          ]
        }
      ]
    }
  };
};
