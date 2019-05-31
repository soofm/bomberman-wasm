const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: path.resolve(__dirname, "src/index.ts"),
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: [ { loader: "ts-loader" } ]
      },
      {
        test: /\.css$/,
        use: [ { loader: "style-loader" }, { loader: "css-loader" } ]
      },
      {
        test: /\.svg$/,
        use: [ { loader: "url-loader", options: { limit: 4096 }} ]
      }
    ],
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src")
    },
    extensions: [ ".wasm", ".ts", ".js" ]
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