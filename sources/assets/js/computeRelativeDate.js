import getRelativeDate from 'helpers/relativeDate.helper'


export default function (container) {
	container
		.querySelectorAll('[data-date]')
		.forEach(element => {
			const date = new Date(element.getAttribute('data-date'))

			const updateRelativeDate = () => element.innerHTML = getRelativeDate(date, updateRelativeDate)

			updateRelativeDate()
		})
}
