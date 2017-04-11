const path = require('path');
var webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const BUILD_DIR = path.join(__dirname, '../dist/client/public');
const APP_DIR = path.join(__dirname, '../dist/client/app');


module.exports = {
    entry: path.join(__dirname, '/src/app-client.js'),

    output: {
        path: BUILD_DIR,
        filename: 'index_bundle.js'
    },

    module: {
        loaders: [{
                test: /\.js$/,
                loader: 'babel-loader',
                exclude: /node_modules/
            },
            {
                test: /\.jsx$/,
                loader: 'babel-loader',
                exclude: /node_modules/
            }
        ]
    },

    plugins: [
      new HtmlWebpackPlugin({
          hash: true,
          template: path.join(__dirname, '/src/index.html'),
          filename: 'index.html',
          inject: 'body'
      }),
      new webpack.optimize.UglifyJsPlugin(),

  ]
}
