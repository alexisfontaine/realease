const { PI, cos } = Math

export default function scrollAnimation (origin, offset, timestamp, now, heightOffset = 0) {
	heightOffset += PI * (now - timestamp) / 250

	if (heightOffset >= PI) return scrollTo(0, origin + offset)

	timestamp = now
	scrollTo(0, origin + .5 * offset * (1 - cos(heightOffset)))
	requestAnimationFrame(now => scrollAnimation(origin, offset, timestamp, now, heightOffset))
}
