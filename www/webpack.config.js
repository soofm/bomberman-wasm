const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

const public = path.resolve(__dirname, "public");
const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: path.resolve(__dirname, "src/index.js"),
  output: {
    path: dist,
    filename: "[name].bundle.js",
  },
  devServer: {
      contentBase: dist
  },
  mode: "development",
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
        template: path.resolve(public, "index.html"),
        favicon: path.resolve(public, "favicon.ico")
    })
  ],
};