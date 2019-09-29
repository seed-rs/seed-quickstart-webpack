const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// Webpack generates `css_classes.rs` with this config.
// This config is used in command `yarn generate:css_classes`.
// See `webpack.config.js` for more info about individual settings.

module.exports = (env, argv) => {
  return {
    entry: path.resolve(__dirname, "../entries/index.css_classes.ts"),
    output: {
      path: path.resolve(__dirname, "../dist"),
      filename: "css_classes.js"
    },
    plugins: [new WebpackBar(), new CleanWebpackPlugin()],
    module: {
      rules: [
        {
          test: /\.(jpg|jpeg|png|woff|woff2|eot|ttf|svg)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                emitFile: false,
                name: "[path][name].[ext]"
              }
            }
          ]
        },
        {
          test: /\.ts$/,
          loader: "ts-loader?configFile=configs/tsconfig.css_classes.json"
        },
        {
          test: /\.css$/,
          use: [
            "style-loader",
            "css-loader",
            {
              loader: "postcss-loader",
              options: {
                config: {
                  ctx: { mode: argv.mode },
                  path: __dirname
                }
              }
            }
          ]
        }
      ]
    }
  };
};
