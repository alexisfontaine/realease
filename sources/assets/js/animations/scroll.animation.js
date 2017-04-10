const { PI, cos } = Math

function scrollAnimation (origin, offset, timestamp, now, heightOffset = 0) {
	heightOffset += PI * (now - timestamp) / 250

	if (heightOffset >= PI) return scrollTo(0, origin + offset)

	timestamp = now
	scrollTo(0, origin + .5 * offset * (1 - cos(heightOffset)))
	requestAnimationFrame(now => scrollAnimation(origin, offset, timestamp, now, heightOffset))
}

export default function () {
	document
		.querySelectorAll('a[href^=\\#]:not([href=\\#])')
		.forEach(anchor => {
			const target = document.querySelector(anchor.hash)

			anchor.addEventListener('click', event => {
				const timestamp = performance.now()
				const offset    = target.getBoundingClientRect().top - scrollY

				event.preventDefault()
				requestAnimationFrame(now => scrollAnimation(scrollY, offset, timestamp, now))
			})
		})
}
