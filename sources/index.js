import Vue from 'vue'
import Application from './components/Application'


const application = new Vue(Application)

export default context => new Promise(resolve => resolve(application))
