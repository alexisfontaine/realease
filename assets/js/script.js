const container = document.querySelector('#releases')
const releases  = Array.from(container.querySelectorAll('.release'))

const { length } = releases
const last       = length - 1

function animate (current = last) {
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
		setTimeout(() => animate(current === 0 ? last : current - 1), 10000)
	})
}

setTimeout(animate, 2000)
