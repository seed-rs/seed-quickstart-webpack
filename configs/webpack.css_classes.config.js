const path = require("path");

const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// Webpack should generate `css_classes.rust` with this config
// It's used in command `yarn generate:css_classes`
// See `webpack.config.js` for more info about individual settings

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
          test: /\.(png|jpg|gif)$/,
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
            { loader: "css-loader", options: { importLoaders: 1 } },
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
