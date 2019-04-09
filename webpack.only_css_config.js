const path = require("path");
const dist = path.resolve(__dirname, "dist");

const CleanWebpackPlugin = require('clean-webpack-plugin');

module.exports = {
  entry: "./index_only_css.ts",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist,
    hot: true,
    host: '0.0.0.0',
    port: 3000
  },
  plugins: [
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
        loader: 'ts-loader?configFile=tsconfig.ignore_index_ts.json',
      }, {
        test: /\.css$/,
        use: [
          'style-loader',
          { loader: 'css-loader', options: { importLoaders: 1 } },
          'postcss-loader'
        ]
      }
    ]
  }
};
