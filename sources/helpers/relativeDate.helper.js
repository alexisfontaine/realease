const { ceil, round } = Math

const terms = [
	{
		time:	45,
		divide:	60,
		prefix:	false,
		suffix:	false,
		text:	'right now'
	},
	{
		time:	90,
		divide:	60,
		text:	'one min'
	},
	{
		time:	3600,
		divide:	60,
		text:	'%d min'
	},
	{
		time:	5400,
		divide:	3600,
		text:	'one hour'
	},
	{
		time:	86400,
		divide:	3600,
		text:	'%d hours'
	},
	{
		time:	129600,
		divide:	86400,
		text:	'one day'
	},
	{
		time:	2592000,
		divide:	86400,
		text:	'%d days'
	},
	{
		time:	3888000,
		divide:	2592000,
		text:	'one month'
	},
	{
		time:	31536000,
		divide:	2592000,
		text:	'%d months'
	},
	{
		time:	47304000,
		divide:	31536000,
		text:	'one year'
	},
	{
		time:	Infinity,
		divide:	31536000,
		text:	'%d years'
	}
]

module.exports = (date, nextTick) => {
	let secondes = (Date.now() - date) / 1000
	let term     = null
	let prefix   = ''
	let suffix   = ''

	for (term of terms)
		if (secondes < term.time) break

	if (secondes > 0) {
		if (term.suffix !== false) suffix = ' ago'
	} else {
		if (term.prefix !== false) prefix = 'in '

		secondes = -secondes
	}

	const units = secondes / term.divide

	setTimeout(() => requestAnimationFrame(nextTick), round(1000 * (ceil(units) * term.divide - secondes)))

	return prefix + term.text.replace('%d', round(units)) + suffix
}
