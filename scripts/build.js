const fs		= require('fs')
const path		= require('path')
const webpack	= require('webpack')
const renderer	= require('vue-server-renderer')
const shell		= require('shelljs')
const minify	= require('./commons/minifier')


const rootDirectory		= path.resolve(__dirname, '..')
const publicDirectory	= path.join(rootDirectory, './public')

shell.rm('-rf', publicDirectory)
shell.mkdir(publicDirectory)

require('./fetch')()
	.then(({ repositories, version }) => {
		const configurations = require('../webpack.config')

		configurations.forEach(configuration => {
			const constants = new webpack.DefinePlugin({
				REPOSITORIES:	JSON.stringify(repositories),
				VERSION:		JSON.stringify(version)
			})

			if (!configuration.plugins) configuration.plugins = [constants]
			else configuration.plugins.push(constants)
		})

		return new Promise((resolve, reject) => webpack(configurations, (error, stats) => {
			if (error) return reject(error)
			if (stats.hasErrors()) return reject(stats.toJson().errors)

			resolve()
		}))
	})
	.then(() => {
		const layout = fs.readFileSync(path.join(rootDirectory, './sources/index.html'), 'utf8')

		return renderer
			.createRenderer({ template: layout })
			.renderToString(require('../public/main.js').default())
			.then(html => {
				fs.writeFileSync(path.join(publicDirectory, './index.html'), minify(html))
				shell.cp('-Rf', path.join(rootDirectory, './assets/*'), publicDirectory)
				shell.mv(path.join(publicDirectory, './images/favicon/favicon.ico'), publicDirectory)
				shell.rm(path.join(publicDirectory, './main.js'))
			})
	})
	.catch(error => {
		console.error(error)
		process.exit(1)
	})
