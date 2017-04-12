import 'core-js/modules/es6.array.for-each'
import 'core-js/modules/es6.array.from'
import 'core-js/modules/web.timers'
import 'core-js/modules/web.dom.iterable'

if (!window.performance) window.performance = { now: Date.now }

NodeList.prototype.forEach = Array.prototype.forEach
