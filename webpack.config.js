const path		= require('path')
const webpack	= require('webpack')
const externals	= require('webpack-node-externals')

const ExtractTextPlugin			= require('extract-text-webpack-plugin')
const OptimizeCssAssetsPlugin	= require('optimize-css-assets-webpack-plugin')


const sourcesDirectory	= path.resolve(__dirname, './sources')
const publicDirectory	= path.resolve(__dirname, './public')
const assetsDirectory	= path.join(sourcesDirectory, './assets')
const stylesDirectory	= path.join(assetsDirectory, './scss')


const mode = 'production'

const sassLoader = {
	loader: 'sass-loader',
	options: { includePaths: [stylesDirectory] }
}

const alias = {
	sources: sourcesDirectory,
	helpers: path.join(sourcesDirectory, './helpers')
}

const rules = [{
	test:		/\.js$/,
	loader:		'babel-loader',
	exclude:	/node_modules\//
}]

module.exports = [
	{
		mode,
		entry: path.join(assetsDirectory, './js/script.js'),
		output: {
			filename:	'script.js',
			path:		publicDirectory
		},
		resolve:	{ alias },
		module:		{ rules }
	},
	{
		mode,
		entry:	path.join(sourcesDirectory, './index.js'),
		target:	'node',
		output: {
			path:			publicDirectory,
			libraryTarget:	'commonjs2'
		},
		externals: externals(),
		node: {
			__dirname:	true,
			__filename:	true
		},
		resolve: {
			extensions: ['.js', '.json', '.vue'],
			alias
		},
		module: {
			rules: rules.concat([
				{
					test:	/\.vue$/,
					loader:	'vue-loader',
					options: {
						loaders: {
							scss: ExtractTextPlugin.extract({
								fallback:	'vue-style-loader',
								use:		['css-loader', sassLoader]
							})
						}
					}
				}
			])
		},
		plugins: [
			new ExtractTextPlugin('styles.css'),
			new OptimizeCssAssetsPlugin
		]
	}
]
