const request		= require('../request')
const markdown		= require('../../../sources/helpers/markdown.helper')
const markdownUtils	= require('../markdown.utils')


const { uniqueAnchor, addHyperlinkTarget } = markdownUtils

const headers = {
	'Content-Type':	'application/json;',
	'User-Agent':	'RealEase'
}

const options = {
	hostname:	'api.github.com',
	path:		'/graphql',
	method:		'POST'
}

module.exports = repositories => {
	const accessToken = process.env.GITHUB_OAUTH_TOKEN

	if (!accessToken)
		throw new Error('You must provide an access token to GitHub GraphQL API!\nSee https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line')

	const requestHeaders	= Object.assign({ Authorization: `Bearer ${accessToken}` }, headers)
	const requestOptions	= Object.assign({ headers: requestHeaders }, options)
	const lastMonth			= Date.now() - 2592000000
	const data				= { query: `{nodes(ids:["${repositories.join('", "')}"]){...on Repository{owner{avatarUrl login resourcePath}releases(last:1){nodes{tag{name}name description publishedAt}}primaryLanguage{name color}stargazers{totalCount}name resourcePath updatedAt homepageUrl description}}}` }

	return request(requestOptions, data)
		.catch(error => {
			if (error.statusCode) throw new Error(getErrorMessage(error.statusCode, error.body))

			throw new Error(`Failed request to GitHub GraphQL API\n${error.message}`)
		})
		.then(response => {
			const result = JSON.parse(response.body)

			if (result.errors) {
				const message = getErrorMessage(response.statusCode, result.errors)

				if (!result.data) throw new Error(message)
				else console.warn(message)
			}

			try {
				return result.data.nodes
					.map(repository => {
						if (!repository || !repository.releases) return

						const release = repository.releases.nodes[0]

						if (!release) return

						repository.release		= release
						repository.language		= repository.primaryLanguage
						repository.stargazers	= repository.stargazers.totalCount
						repository.updatedAt	= new Date(repository.updatedAt)
						repository.publishedAt	= new Date(release.publishedAt)
						repository.description	= addHyperlinkTarget(markdown(repository.description))
						release.description		= addHyperlinkTarget(uniqueAnchor(markdown(release.description), repository))
						release.tag				= release.tag && release.tag.name

						delete repository.releases
						delete repository.primaryLanguage
						delete release.publishedAt

						return repository
					})
					.filter(repository => repository && repository.publishedAt > lastMonth)
					.sort((a, b) => a.publishedAt - b.publishedAt)
			} catch (error) {
				throw new Error(`Failed parsing response from GitHub GraphQL API\n${error.message}`)
			}
		})
}

function getErrorMessage(statusCode, error) {
	return `GitHub GraphQL API returned ${statusCode} status code\n${typeof error === 'string' ? error : JSON.stringify(error)}`
}
