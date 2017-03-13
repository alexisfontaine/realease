const path = require('path')
const webpack = require('webpack')

module.exports = {
    entry: path.resolve(__dirname, './sources/index.js'),
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
                loader: 'vue-loader',
                options: {
                    postcss: [require('autoprefixer')({ browsers: ['last 3 versions'] })]
                }
            },
            {
                test: /\.js$/,
                loader: 'babel-loader',
                include: path.resolve(__dirname, './sources'),
                exclude: /node_modules\//,
            }
        ]
    },
    plugins: []
}
