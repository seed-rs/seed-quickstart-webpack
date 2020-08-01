const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// Webpack generates `css_classes.rs` with this config.
// This config is used in command `yarn generate:css_classes`.
// See `webpack.config.js` for more info about individual settings.

module.exports = (env, argv) => {
  return {
    entry: path.resolve(__dirname, "./styles/app.scss"),
    output: {
      path: path.resolve(__dirname, "../dist"),
      filename: "css_classes.js"
    },
    plugins: [new WebpackBar(), new CleanWebpackPlugin()],
    module: {
      rules: [
        {
          test: /\.(jpg|jpeg|png|woff(2)?|eot|ttf|svg)(\?[a-z0-9]+)?$/,
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
          test: /\.s[ac]ss$/i,
          use: [
            "style-loader",
            'css-loader',
            {
              loader: "postcss-loader",
              options: {
                config: {
                  ctx: { mode: argv.mode },
                  path: __dirname,
                }
              }
            },
            'sass-loader',
          ]
        }
      ]
    }
  };
};
