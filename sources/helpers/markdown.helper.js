const marked	= require('marked')
const highlight	= require('highlight.js')


highlight.configure({
	tabReplace:		'    ',
	classPrefix:	''
})

marked.setOptions({
	highlight: (code, lang) => lang ? highlight.highlight(lang, code, true).value : code
})

module.exports = marked
