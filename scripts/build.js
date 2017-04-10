const fs       = require('fs')
const path     = require('path')
const webpack  = require('webpack')
const renderer = require('vue-server-renderer')
const shell    = require('shelljs')
const minify   = require('./commons/minifier')


const rootDirectory   = path.resolve(__dirname, '..')
const publicDirectory = path.join(rootDirectory, './public')

shell.rm('-rf', publicDirectory)
shell.mkdir(publicDirectory)

require('./fetch')()
	.then(repositories => {
		const configurations = require('../webpack.config')
		const plugin         = new webpack.DefinePlugin({ REPOSITORIES: JSON.stringify(repositories) })

		configurations.forEach(configuration => configuration.plugins.push(plugin))

		return new Promise((resolve, reject) => webpack(configurations, (error, stats) => {
			if (error) return reject(error)
			if (stats.hasErrors()) return reject(stats.toJson().errors)

			resolve()
		}))
	})
	.then(() => {
		const bundle = path.join(publicDirectory, './bundle.js')
		const layout = fs.readFileSync(path.join(rootDirectory, './sources/index.html'), 'utf8')
		const entry  = fs.readFileSync(bundle, 'utf8')

		global.Vue = require('vue')

		return new Promise((resolve, reject) => renderer
			.createBundleRenderer(entry, { template: layout })
			.renderToString((error, html) => {
				if (error) return reject(error)

				fs.writeFileSync(path.join(publicDirectory, './index.html'), minify(html))
				shell.cp('-Rf', path.join(rootDirectory, './assets/*'), publicDirectory)
				shell.mv(path.join(publicDirectory, './images/favicon/favicon.ico'), publicDirectory)
				shell.rm(bundle)
				resolve()
			}))
	})
	.catch(error => {
		console.error(error)
		process.exit(1)
	})
