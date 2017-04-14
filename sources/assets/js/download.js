export default function (container) {
	const platform = navigator.platform.toLowerCase()
	const buttons  = Array.from(container.querySelectorAll('.button[id^=download_]'))

	let id = 'download_other'

	if (platform.includes('win')) id = 'download_windows'
	else if (platform.includes('mac')) id = 'download_macos'
	else container.querySelector('#download').style.display = 'none'

	buttons.find(button => button.id === id).classList.add('active')
}
