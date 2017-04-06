const { PI, cos, ceil, round } = Math

const container = document.querySelector('#releases')

/* Relative Date */
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

function getRelativeDate (date, nextTick) {
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

container
	.querySelectorAll('[data-date]')
	.forEach(element => {
		const date = new Date(element.getAttribute('data-date'))
		const updateRelativeDate = () => element.innerHTML = getRelativeDate(date, updateRelativeDate)

		updateRelativeDate()
	})

/* Animations */
const releases   = container.querySelectorAll('.release')
const { length } = releases
const last       = length - 1

function flipSlideAnimation (current = last) {
	requestAnimationFrame(() => {
		releases.forEach((release, index) => {
			release.classList.toggle('flip', current === index)
			release.style.order = current > index
				? length + index - current
				: index - current
		})

		container.style.transform = `translateY(-${releases[current].clientHeight}px)`
		container.classList.remove('translate')
		requestAnimationFrame(() => container.classList.add('translate'))
		setTimeout(() => flipSlideAnimation(current === 0 ? last : current - 1), 10000)
	})
}

function scrollAnimation (origin, offset, timestamp, now, heightOffset = 0) {
	heightOffset += PI * (now - timestamp) / 250

	if (heightOffset >= PI) return scrollTo(0, origin + offset)

	timestamp = now
	scrollTo(0, origin + .5 * offset * (1 - cos(heightOffset)))
	requestAnimationFrame(now => scrollAnimation(origin, offset, timestamp, now, heightOffset))
}

setTimeout(flipSlideAnimation, 2000)

document
	.querySelectorAll('a[href^=\\#]:not([href=\\#])')
	.forEach(anchor => {
		const target = document.querySelector(anchor.hash)

		anchor.addEventListener('click', event => {
			const timestamp = performance.now()
			const offset = target.getBoundingClientRect().top - scrollY

			event.preventDefault()
			requestAnimationFrame(now => scrollAnimation(scrollY, offset, timestamp, now))
		})
	})
