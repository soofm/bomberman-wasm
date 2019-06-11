const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require('path');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: path.resolve(__dirname, "js/index.ts"),
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
        test: /\.(png|svg)$/,
        use: [ { loader: "url-loader", options: { limit: 4096 }} ]
      }
    ],
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "js")
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
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "public", "index.html"),
        favicon: path.resolve(__dirname, "public", "favicon.ico")
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate"),
      // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
      // forceMode: 'release'
    }),
  ],
};