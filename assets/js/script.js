const container = document.querySelector('#releases')
const releases  = Array.from(container.querySelectorAll('.release'))

const { length } = releases
const last       = length - 1

function animate (current = last) {
	releases.forEach((release, index) => {
		release.classList.toggle('flip', current === index)
		release.style.order = current > index
			? length + index - current
			: index - current
	})

	container.style.transform = `translateY(-${releases[current].clientHeight}px)`
	container.classList.remove('translate')
	setTimeout(() => container.classList.add('translate'), 0)
	setTimeout(() => animate(current === 0 ? last : current - 1), 8000)
}

setTimeout(() => animate(), 2000)
