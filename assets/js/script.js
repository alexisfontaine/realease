const { PI, cos } = Math

const container = document.querySelector('#releases')
const releases  = container.querySelectorAll('.release')

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
