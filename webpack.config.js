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
      filename: '[name].[contenthash].js',
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
    plugins: [
      // Show compilation progress bar in console.
      new WebpackBar(),
      // Clean `dist` folder before compilation.
      new CleanWebpackPlugin(),
      // Extract CSS styles into a file.
      new MiniCssExtractPlugin({
        filename: '[name].[contenthash].css'
      }),
      new HtmlWebpackPlugin({
        title: "Seed App",
        template: "assets/index.html",
        scriptLoading: "defer",
        inject: "body",
        meta: {
          viewport: "width=device-width, initial-scale=1.0, user-scalable=1",
        },
        favicon: "assets/favicons/favicon.ico",
      }),
      // Compile Rust.
      new WasmPackPlugin({
        crateDirectory: __dirname
      }),

      // You can find files from folder `../assets` on url `http://my-site.com/assets/`.
      // And favicons in the root.
      new CopyWebpackPlugin([
        {
          from: "assets",
          to: "assets",
          ignore: ['index.*', 'css/**/*', 'favicons/**/*']
        },
        {
          from: "assets/favicons",
          to: ""
        },
        {
          from: "assets/css/fontawesome/webfonts",
          to: "assets/css/fontawesome/webfonts"
        }
      ]),
    ],
    // Webpack try to guess how to resolve imports in this order:
    resolve: {
      extensions: [".ts", ".js", ".wasm"],
      alias: {
        crate: __dirname,
      }
    },
    module: {
      rules: [
        {
          test: /\.(jpg|jpeg|png|woff(2)?|eot|ttf|svg)(\?[a-z0-9]+)?$/,
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
          loader: "ts-loader?configFile=tsconfig.json"
        },
        {
          test: /\.s[ac]ss$/i,
          use: [
            MiniCssExtractPlugin.loader,
            'css-loader',
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
            },
            'sass-loader',
          ]
        }
      ]
    },
  };
};
