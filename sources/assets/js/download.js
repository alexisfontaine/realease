export default function (container) {
	const platform	= navigator.platform.toLowerCase()
	const userAgent	= navigator.userAgent.toLocaleLowerCase()
	const buttons	= Array.from(container.querySelectorAll('.button[id^=download_]'))

	let id = 'download_other'

	if (platform.includes('win')) id = `download_windows${userAgent.includes('wow64') || userAgent.includes('win64') ? 64 : 32}`
	else if (platform.includes('mac')) id = 'download_macos'
	else container.querySelector('#download').style.display = 'none'

	buttons.find(button => button.id === id).classList.add('active')
}
