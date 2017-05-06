const fs   = require('fs')

const configuration     = require('../configuration')
const fetchRepositories = require('./commons/fetch/repositories')
const fetchVersion      = require('./commons/fetch/version')


function run () {
	return Promise.all([
		fetchRepositories(configuration.repositories),
		fetchVersion()
	])
		.then(([repositories, version]) => ({ repositories, version }))
		.catch(error => {
			console.error(error)
			process.exit(1)
		})
}

if (require.main === module) run()
	.then(data => {
		const { resolve } = require('path')

		fs.writeFile(resolve(__dirname, '../public/data.json'), JSON.stringify(data), error => {
			if (!error) return

			console.error(error)
			process.exit(1)
		})
	})
else module.exports = run
