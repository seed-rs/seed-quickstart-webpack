const path = require("path");
const dist = path.resolve(__dirname, "../dist");

const WebpackBar = require("webpackbar");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const Critters = require('critters-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = (env, argv) => {
  return {
    entry: {
      // Bundle root with name `app.js`.
      app: path.resolve(__dirname, "../entries/index.ts")
    },
    output: {
      // You can deploy your site from this folder (after build with e.g. `yarn build:release`)
      path: dist,
      filename:'[name].[contenthash].js'
    },
    devServer: {
      contentBase: dist,
      // You can connect to dev server from devices in your network (e.g. 192.168.0.3:8000).
      host: "0.0.0.0",
      port: 8000,
      noInfo: true,
      stats: "errors-only",
      overlay: {
        warnings: true,
        errors: true
      },
      historyApiFallback: true,
    },
    plugins: [
      // Show compilation progress bar in console.
      new WebpackBar(),
      // Clean `dist` folder before compilation.
      new CleanWebpackPlugin(),
      // Extract CSS styles into a file.
      new MiniCssExtractPlugin({
        filename:'[name].[contenthash].css'
      }),
      // Add scripts, css, ... to html template.
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "../entries/index.hbs")
      }),
      // Inline the critical part of styles, preload remainder.
      new Critters({
        logLevel: "warn",
        // https://github.com/GoogleChromeLabs/critters/issues/34
        pruneSource: false,
      }),
      // Compile Rust.
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../crate"),
        outDir: path.resolve(__dirname, "../crate/pkg")
      }),

      // You can find files from folder `../static` on url `http://my-site.com/static/`.
      // And favicons in the root.
      new CopyWebpackPlugin([
        {
          from: "static",
          to: "static"
        },
        {
          from: "favicons",
          to: ""
        }
      ]),
    ],
    // Webpack try to guess how to resolve imports in this order:
    resolve: {
      extensions: [".ts", ".js", ".wasm"],
      alias: {
        crate: path.resolve(__dirname, "../crate")
      }
    },
    module: {
      rules: [
        {
          test: /\.hbs$/,
          use: [
            {
              loader: "handlebars-loader",
              options: {
                rootRelative: './templates/'
              }
            }
          ]
        },
        {
          test: /\.(jpg|jpeg|png|woff|woff2|eot|ttf|svg)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                // Don't copy files to `dist`, we do it through `CopyWebpackPlugin` (see above)
                // - we only want to resolve urls to these files.
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
            MiniCssExtractPlugin.loader,
            "css-loader",
            {
              loader: "postcss-loader",
              options: {
                config: {
                  // Path to postcss.config.js.
                  path: __dirname,
                  // Pass mode into `postcss.config.js` (see more info in that file).
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
