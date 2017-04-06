import Vue from 'vue'
import Application from './components/Application'

import numberFilter from './filters/number.filter'


Vue.filter('number', numberFilter)

const application = new Vue(Application)

export default context => new Promise(resolve => resolve(application))
