import Vue from 'vue'
import Application from './components/Application'

import relativeDateFilter from './filters/relative.date.filter'
import numberFilter from './filters/number.filter'


Vue.filter('relativeDate', relativeDateFilter)
Vue.filter('number', numberFilter)

const application = new Vue(Application)

export default context => new Promise(resolve => resolve(application))
