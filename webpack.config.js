const path = require("path");
const dist = path.resolve(__dirname, "dist");

const WebpackBar = require("webpackbar");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = (env, argv) => {
  return {
    performance: {
      // Don't break compilation because of WASM file bigger than 244 KB.
      hints: false
    },
    entry: {
      // Bundle root with name `app.js`.
      app: path.resolve(__dirname, "index.ts")
    },
    output: {
      // You can change it to e.g. `/ui/`, but also edit `historyApiFallback` below and `<base href..`> in `index.hbs`.
      publicPath: '/',
      // You can deploy your site from this folder (after build with e.g. `yarn build:release`)
      path: dist,
      filename: '[name].[contenthash].js'
    },
    devServer: {
      contentBase: dist,
      // You can connect to dev server from devices in your network (e.g. 192.168.0.3:8000).
      host: "0.0.0.0",
      port: 8000,
      // Route everything to index to support SPA. It should be the same like `publicPath` above.
      historyApiFallback: {
        index: '/'
      },
      noInfo: true,
      stats: "errors-only",
      overlay: {
        // Commented to prevent error:
        // `./crate/pkg/index_bg.js 382:14-53   Critical dependency: the request of a dependency is an expression`
        // warnings: true,
        errors: true
      },
    },
    devtool: "eval",
    experiments: {
      asyncWebAssembly: true,
    },
    plugins: [
      // Show compilation progress bar in console.
      new WebpackBar(),
      // Clean `dist` folder before compilation.
      new CleanWebpackPlugin(),
      // Extract CSS styles into a file.
      new MiniCssExtractPlugin({
        filename: '[name].[contenthash].css'
      }),
      // Add scripts, css, ... to html template.
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "static/index.hbs")
      }),
      // Compile Rust.
      new WasmPackPlugin({
        crateDirectory: __dirname
      }),

      // You can find files from folder `../static` on url `http://my-site.com/static/`.
      // And favicons in the root.
      new CopyWebpackPlugin({
        patterns: [
          {
            from: "static",
            to: "static"
          },
          {
            from: "favicons",
            to: ""
          }
        ]
      }),
    ],
    // Webpack try to guess how to resolve imports in this order:
    resolve: {
      extensions: [".ts", ".js", ".wasm"],
      alias: {
        crate: __dirname
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
          use: [
            {
              loader: "ts-loader",
              options: {
                configFile: "tsconfig.css_classes.json"
              }
            }
          ]

        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            "css-loader",
            {
              loader: "postcss-loader",
              options: {
                postcssOptions: {
                  config: path.resolve(__dirname, "postcss.config.js"),
                }
              },
            },
          ]
        }
      ]
    }
  };
};
