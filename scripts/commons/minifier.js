const { minify } = require('html-minifier')


const options = {
	minifyCSS:                 true,
	minifyJS:                  true,
	removeComments:            true,
	removeRedundantAttributes: true,
	removeAttributeQuotes:     true,
	collapseWhitespace:        true,
	collapseBooleanAttributes: true
}

module.exports = string => minify(string
	.replace(/ (data-vue-ssr-id|server-rendered)(="[^"]+")?/g, '')
	.replace(/<\/style><style>/g, ''), options)
