const path    = require('path')
const webpack = require('webpack')

const ExtractTextPlugin       = require('extract-text-webpack-plugin')
const BabiliPlugin            = require('babili-webpack-plugin')
const OptimizeCssAssetsPlugin = require('optimize-css-assets-webpack-plugin')


const sourcesDirectory = path.resolve(__dirname, './sources')
const publicDirectory  = path.resolve(__dirname, './public')
const assetsDirectory  = path.join(sourcesDirectory, './assets')
const stylesDirectory  = path.join(assetsDirectory, './scss')

const cssLoaders  = ['css-loader', 'postcss-loader']
const sassLoaders = cssLoaders.concat([{
	loader: 'sass-loader',
	options: {
		includePaths: [stylesDirectory]
	}
}])

const alias = {
	sources: sourcesDirectory,
	helpers: path.join(sourcesDirectory, './helpers')
}

const rules = [{
	test:    /\.js$/,
	loader:  'babel-loader',
	include: sourcesDirectory,
	exclude: /node_modules\//
}]

const plugins = [
	new webpack.DefinePlugin({ 'process.env.NODE_ENV': '"production"' })
]

module.exports = [
	{
		entry: {
			script: path.join(assetsDirectory, './js/script.js')
		},
		output: {
			path:       publicDirectory,
			filename:   '[name].js',
			publicPath: '/'
		},
		resolve: { alias },
		module:  { rules },
		plugins: plugins.concat([new webpack.optimize.UglifyJsPlugin()])
	},
	{
		entry: {
			bundle: [
				path.join(stylesDirectory, './styles.scss'),
				path.join(sourcesDirectory, './index.js')
			]
		},
		target: 'node',
		output: {
			path:          publicDirectory,
			libraryTarget: 'commonjs2',
			filename:      '[name].js',
			publicPath:    '/'
		},
		node: {
			__dirname:  true,
			__filename: true
		},
		resolve: {
			extensions: ['.js', '.json', '.vue'],
			alias
		},
		module: {
			rules: rules.concat([
				{
					test:   /\.vue$/,
					loader: 'vue-loader',
					options: {
						loaders: {
							css: ExtractTextPlugin.extract({
								fallback: 'vue-style-loader',
								use:       cssLoaders
							}),
							scss: ExtractTextPlugin.extract({
								fallback: 'vue-style-loader',
								use:       sassLoaders
							})
						}
					}
				},
				{
					test: /\.css$/,
					use: ExtractTextPlugin.extract({
						fallback: 'style-loader',
						use:      cssLoaders
					})
				},
				{
					test: /\.scss$/,
					use: ExtractTextPlugin.extract({
						fallback: 'style-loader',
						use:      sassLoaders
					})
				}
			])
		},
		plugins: plugins.concat([
			new ExtractTextPlugin('styles.css'),
			new OptimizeCssAssetsPlugin({ canPrint: false }),
			new BabiliPlugin()
		])
	}
]
