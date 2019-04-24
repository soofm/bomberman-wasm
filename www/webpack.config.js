const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./index.js",
  output: {
    path: dist,
    filename: "index.bundle.js",
  },
  devServer: {
      contentBase: dist
  },
  mode: "development",
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
        template: "index.html"
    })
  ],
};