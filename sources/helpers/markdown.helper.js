const marked	= require('marked')
const highlight	= require('highlight.js')
const keywords	= require('emojis-keywords')
const emojis	= require('emojis-list')


highlight.configure({
	tabReplace:		'    ',
	classPrefix:	''
})

marked.setOptions({
	highlight: (code, lang) => lang ? highlight.highlight(lang, code, true).value : code
})

module.exports = markdown => markdown
	? marked(markdown)
		.replace(/:([a-z0-9_-]+:‍?:)*[a-z0-9_-]+(:‍?(♀️|♂️|⚕️)?)/g, keyword => {
			const index = keywords.indexOf(keyword)

			return index === -1 ? keyword : emojis[index]
		})
	: ''
