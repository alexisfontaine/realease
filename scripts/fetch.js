const accessToken = process.env.GITHUB_OAUTH_TOKEN

if (!accessToken) {
	console.error('You must provide an access token to GitHub GraphQL API!')
	console.error('See https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/')
	process.exit(1)
}


const fs    = require('fs')
const https = require('https')
const path  = require('path')

const markdown      = require('../sources/helpers/markdown.helper')
const configuration = require('../configuration')


const lastMonth = Date.now() - 2592000000

const headers = {
	Authorization: `Bearer ${accessToken}`,
	'Content-Type': 'application/json;',
	'User-Agent': 'RealEase'
}

const options = {
	hostname: 'api.github.com',
	path: '/graphql',
	method: 'POST'
}

function uniqueAnchor (string, { owner, name }) {
	const prefix = `${owner.login}-${name}-`

	return string
		.replace(/id="([^"]+)"/g, `id="${prefix}$1"`)
		.replace(/href="#([^"]+)"/g, `href="#${prefix}$1"`)
}

function addHyperlinkTarget (string) {
	return string.replace(/(href="https?:\/\/[^"]+")/g, '$1 target="_blank"')
}

function run () {
	return Promise.all([
		new Promise((resolve, reject) => {
			const data           = JSON.stringify({ query: `{nodes(ids:["${configuration.repositories.join('", "')}"]){...on Repository{owner{avatarUrl login path}releases(last:1){nodes{tag{name}name description publishedAt}}primaryLanguage{name color}stargazers{totalCount}name path updatedAt homepageUrl description}}}` })
			const requestHeaders = Object.assign({ 'Content-Length': data.length }, headers)
			const requestOptions = Object.assign({ headers: requestHeaders }, options)

			const request = https.request(requestOptions, response => {
				response.body = ''
				response.setEncoding('utf8')
				response.on('data', chunk => response.body += chunk)
				response.on('end', () => {
					const body = JSON.parse(response.body)

					if (response.statusCode < 400 && !body.errors) return resolve(JSON.parse(response.body).data.nodes)

					reject(`GitHub GraphQL API returned ${response.statusCode} status code\n${response.body}`)
				})
			})

			request.on('error', error => reject(`Error retrieving repositories information\n${error.message}`))
			request.write(data)
			request.end()
		})
	])
		.then(([repositories]) => repositories
			.map(repository => {
				if (!repository || !repository.releases) return

				const release = repository.releases.nodes[0]

				if (!release) return

				repository.release     = release
				repository.language    = repository.primaryLanguage
				repository.stargazers  = repository.stargazers.totalCount
				repository.updatedAt   = new Date(repository.updatedAt)
				repository.publishedAt = new Date(release.publishedAt)
				repository.description = addHyperlinkTarget(markdown(repository.description))
				release.description    = addHyperlinkTarget(uniqueAnchor(markdown(release.description), repository))
				release.tag            = release.tag && release.tag.name

				delete repository.releases
				delete repository.primaryLanguage
				delete release.publishedAt

				return repository
			})
			.filter(repository => repository && repository.publishedAt > lastMonth)
			.sort((a, b) => a.publishedAt - b.publishedAt))
		.catch(error => {
			console.error(error)
			process.exit(1)
		})
}

if (require.main === module) run()
	.then(repositories => {
		const output = path.resolve(__dirname, '../public/repositories.json')

		fs.writeFile(output, JSON.stringify(repositories), error => {
			if (!error) return

			console.error(error)
			process.exit(1)
		})
	})
else module.exports = run
