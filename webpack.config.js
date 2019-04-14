const path = require("path");
const dist = path.resolve(__dirname, "dist");

const WebpackBar = require('webpackbar');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const CleanWebpackPlugin = require('clean-webpack-plugin');

module.exports = (env, argv) => {
  return {
    entry: {
      app: "./index.ts"
    },
    output: {
      path: dist,
    },
    devServer: {
      contentBase: dist,
      hot: true,
      host: '0.0.0.0',
      port: 3000,
      noInfo: true,
      stats: 'errors-only',
      // doesn't work?
      overlay: {
        warnings: true,
        errors: true
      }
    },
    plugins: [
      new WebpackBar(),
      new CleanWebpackPlugin(),
      new HtmlWebpackPlugin({
        template: 'index.html',
        hash: true
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "crate"),
        // It fails with "index out of bounds" in `development` mode
        // when there are many constants in view template.
        // I don't know why (?)
        forceMode: "production",
      }),
      // Uncomment if you have problems with Edge and polyfill in index.html isn't enough
      //
      // Have this example work in Edge which doesn't ship `TextEncoder` or
      // `TextDecoder` at this time.
      // new webpack.ProvidePlugin({
      //   TextDecoder: ['text-encoding', 'TextDecoder'],
      //   TextEncoder: ['text-encoding', 'TextEncoder']
      // }),
      new CopyWebpackPlugin([
        {
          from: 'static',
          to: 'static'
        }
      ])
    ],
    resolve: {
      extensions: [ ".ts", ".js", '.wasm']
    },
    module: {
      rules: [
        {
          test: /\.(png|jpg|gif)$/,
          use: [
            {
              loader: 'file-loader',
              options: {
                emitFile: false,
                name: '[path][name].[ext]'
              },
            },
          ],
        },
        {
          test: /\.ts$/,
          loader: 'ts-loader',
        }, {
          test: /\.css$/,
          use: [
            'style-loader',
            { loader: 'css-loader', options: { importLoaders: 1 } },
            {
              loader: 'postcss-loader',
              options: {
                config: {
                  ctx: { mode: argv.mode }
                }
              }
            }
          ]
        }
      ]
    }
  }
};
