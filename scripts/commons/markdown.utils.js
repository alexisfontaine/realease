const project = require('../../package.json')


const repository = {
	owner:	project.author,
	name:	project.name
}

function uniqueAnchor (string, { owner, name } = repository) {
	const prefix = `${owner.login}-${name}-`

	return string
		.replace(/id="([^"]+)"/g, `id="${prefix}$1"`)
		.replace(/href="#([^"]+)"/g, `href="#${prefix}$1"`)
}

function addHyperlinkTarget (string) {
	return string.replace(/(href="https?:\/\/[^"]+")/g, '$1 target="_blank"')
}

module.exports = { uniqueAnchor, addHyperlinkTarget }
