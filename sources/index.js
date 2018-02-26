import Vue from 'vue'
import Application from './components/Application'

import numberFilter from './filters/number.filter'


Vue.filter('number', numberFilter)

export default context => new Vue(Application)
