const path = require("path");
const dist = path.resolve(__dirname, "dist");

const WebpackBar = require('webpackbar');
const CleanWebpackPlugin = require('clean-webpack-plugin');

module.exports = (env, argv) => {
  return {
    entry: "./index.css_classes.ts",
    output: {
      path: dist,
      filename: "css_classes.js"
    },
    plugins: [
      new WebpackBar(),
      new CleanWebpackPlugin()
    ],
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
          loader: 'ts-loader?configFile=tsconfig.css_classes.json',
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
