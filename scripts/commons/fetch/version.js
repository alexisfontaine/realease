const request       = require('../request')
const markdown      = require('../../../sources/helpers/markdown.helper')
const markdownUtils = require('../markdown.utils')


const { uniqueAnchor, addHyperlinkTarget } = markdownUtils

const options = {
	headers: {
		'Content-Type': 'application/json;',
		'User-Agent':   'RealEase'
	},
	hostname: 'realease.herokuapp.com',
	path:     '/api/version/latest',
	method:   'GET'
}

function fetchVersion (requestOptions = options) {
	return request(requestOptions)
		.catch(error => {
			if (error.statusCode) throw new Error(`Squirrel server returned ${error.statusCode} status code\n${error.body}`)

			throw new Error(`Failed request to Squirrel server\n${error.message}`)
		})
		.then(response => {
			if (response.statusCode >= 300) {
				const { location } = response.headers

				if (location && location !== requestOptions.path) {
					console.info(`Redirecting to ${requestOptions.hostname}${location} ...`)

					return fetchVersion(Object.assign({}, requestOptions, { path: location }))
				}
			}

			try {
				const result = JSON.parse(response.body)

				return {
					tag:         result.version,
					title:       result.name,
					publishedAt: result.pub_date,
					description: addHyperlinkTarget(uniqueAnchor(markdown(result.notes))),
					assets:      result.files
				}
			} catch (error) {
				throw new Error(`Failed parsing response from Squirrel server\n${error.message}`)
			}
		})
}

module.exports = fetchVersion
