const https = require('https')


module.exports = (options, data) => new Promise((resolve, reject) => {
	const requestHeaders = Object.assign({}, options.headers)
	const requestOptions = Object.assign({}, options, { headers: requestHeaders })

	if (data) {
		data = JSON.stringify(data)
		requestHeaders['Content-Length'] = data.length
	}

	const request = https.request(requestOptions, response => {
		response.body = ''
		response.setEncoding('utf8')
		response.on('data', chunk => response.body += chunk)
		response.on('end', () => (response.statusCode < 400 ? resolve : reject)(response))
	})

	request.on('error', reject)

	if (data) request.write(data)

	request.end()
})
