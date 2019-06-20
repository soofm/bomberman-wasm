const CleanWebpackPlugin = require("clean-webpack-plugin")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
const MiniCssExtractPlugin = require("mini-css-extract-plugin")
const path = require('path')

const dist = path.resolve(__dirname, "dist")

module.exports = (env) => {
  const isDevelopment = env === "development";
  const config = {
    mode: isDevelopment ? "development" : "production",
    entry: path.resolve(__dirname, "js/index.ts"),
    module: {
      rules: [
        {
          test: /\.ts$/,
          use: [ "ts-loader" ]
        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            "css-loader"
          ]
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
    plugins: [
      new CleanWebpackPlugin(),
      new HtmlWebpackPlugin({
          template: path.resolve(__dirname, "public", "index.html"),
          favicon: path.resolve(__dirname, "public", "favicon.ico")
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "crate"),
        // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
        forceMode: isDevelopment ? undefined : "release"
      }),
      new MiniCssExtractPlugin()
    ],
  }

  if (isDevelopment) {
    config.devServer = {
      contentBase: dist
    }
  }

  return config
}
