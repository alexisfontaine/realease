import './polyfills'

import computeRelativeDate from './computeRelativeDate'
import download from './download'
import flipSlideAnimation from './animations/flipSlide.animation'
import scrollAnimation from './animations/scroll.animation'


const introduction	= document.querySelector('#introduction')
const releases		= document.querySelector('#releases')

computeRelativeDate(releases)
download(introduction)
flipSlideAnimation(releases)
scrollAnimation()
