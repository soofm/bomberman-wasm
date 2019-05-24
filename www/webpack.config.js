const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: path.resolve(__dirname, "src/index.js"),
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [{ loader: 'style-loader' }, { loader: 'css-loader' }],
      },
    ],
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src")
    }
  },
  output: {
    path: dist,
    filename: "[name].bundle.js",
  },
  devServer: {
      contentBase: dist
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "public", "index.html"),
        favicon: path.resolve(__dirname, "public", "favicon.ico")
    })
  ],
};