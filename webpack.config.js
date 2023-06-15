const path = require('path');
const HTMLWebpackPlugins = require('html-webpack-plugin');

module.exports = {
    entry: './public/main.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "index.js",
    },
    plugins: [
        new HTMLWebpackPlugins({
            template: "./public/index.html",
        })
    ]
}