const path = require('path')
const webpack = require('webpack')


const sourcesDirectory = path.resolve(__dirname, './sources')

module.exports = {
    entry: path.join(sourcesDirectory, './index.js'),
    target: 'node',
    output: {
        libraryTarget: 'commonjs2',
        path: path.resolve(__dirname, './public'),
        filename: 'bundle.js',
        publicPath: '/'
    },
    node: {
        __dirname: true,
        __filename: true
    },
    resolve: {
        extensions: ['.js', '.json', '.vue']
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: 'vue-loader'
            },
            {
                test: /\.js$/,
                loader: 'babel-loader',
                include: sourcesDirectory,
                exclude: /node_modules\//,
            }
        ]
    },
    plugins: []
}
