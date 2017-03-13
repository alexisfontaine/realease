const accessToken = process.env.GITHUB_OAUTH_TOKEN

if (!accessToken) {
	console.error('You must provide an access token to GitHub GraphQL API!')
	console.error('See https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/')
	process.exit(1)
}


const fs    = require('fs')
const https = require('https')
const path  = require('path')

const markdown     = require('../sources/helpers/markdown.helper')
const repositories = require('../config/repositories')


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

function run () {
	return Promise.all(repositories.map(repository => new Promise((resolve, reject) => {
		const data           = JSON.stringify({ query: `query{repository(owner:"${repository.split('/').join('"name:"')}"){owner{avatarURL login path}releases(last:1){nodes{tag{name}name description}}primaryLanguage{name color}stargazers{totalCount}name path updatedAt homepageURL description}}` })
		const requestHeaders = Object.assign({ 'Content-Length': data.length }, headers)
		const requestOptions = Object.assign({ headers: requestHeaders }, options)

		const request = https.request(requestOptions, response => {
			response.body = ''
			response.setEncoding('utf8')
			response.on('data', chunk => response.body += chunk)
			response.on('end', response.statusCode === 200
				? () => resolve(JSON.parse(response.body).data.repository)
				: () => reject(`${repository}: GitHub GraphQL API returned ${response.statusCode} status code\n${response.body}`))
		})

		request.on('error', error => reject(`${repository}: Error retrieving repository information\n${error.message}`))
		request.write(data)
		request.end()
	})))
		.then(repositories => repositories.map(repository => {
			const release = repository.releases.nodes[0]

			repository.release     = release
			repository.language    = repository.primaryLanguage
			repository.stargazers  = repository.stargazers.totalCount
			repository.description = markdown(repository.description)
			release.description    = markdown(release.description)
			release.tag            = release.tag.name

			delete repository.releases
			delete repository.primaryLanguage

			return repository
		}))
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
