import computeRelativeDate from './computeRelativeDate'
import flipSlideAnimation from './animations/flipSlide.animation'
import scrollAnimation from './animations/scroll.animation'


const container = document.querySelector('#releases')

computeRelativeDate(container)
flipSlideAnimation(container)
scrollAnimation()
