import getRelativeDate from 'helpers/relativeDate.helper'
import flipSlideAnimation from './animations/flipSlide.animation'
import scrollAnimation from './animations/scroll.animation'

const container = document.querySelector('#releases')


/* Relative Date */
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

setTimeout(flipSlideAnimation, 2000)

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
