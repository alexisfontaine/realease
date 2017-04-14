<template>
    <section class="download">
		<div class="container">
			<h2 id="downloads">Downloads</h2>

			<table>
				<tbody>
					<tr>
						<td></td>
						<td>32-bit</td>
						<td>64-bit</td>
					</tr>
					<tr>
						<td><b>macOS</b> (.dmg)</td>
						<td></td>
						<td>
							<a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-x64.dmg" class="link" download>Download</a>
						</td>
					</tr>
					<tr>
						<td><b>Windows</b> (.exe)</td>
						<td>
							<a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-x86.exe" class="link" download>Download</a>
						</td>
						<td>
							<a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-x64.exe" class="link" download>Download</a>
						</td>
					</tr>
					<tr>
						<td><b>Windows</b> (.zip)</td>
						<td>
							<a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-win32-x86.zip" class="link" download>Download</a>
						</td>
						<td>
							<a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-win32-x64.zip" class="link" download>Download</a>
						</td>
					</tr>
					<tr>
						<td><b>Debian</b> (.deb)</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
					</tr>
					<tr>
						<td><b>Fedora</b> (.rpm)</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
					</tr>
					<tr>
						<td><b>Other Linux</b> (.AppImage)</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
						<td>
							<a class="link disabled">Coming soon</a>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</section>
</template>

<script>
	export default { name: 'download' }
</script>

<style lang="scss">
	@import 'colors';


	.download{
		position: relative;
		overflow: visible;
		background-color: $dark-blue;
		z-index: 0;
	}

	.download::before, .download::after{
		content: '';
		position: absolute;
		top: 0; right: 0; bottom: 0; left: 0;
		background-color: inherit;
		z-index: -1;
	}

	.download::before{
		transform-origin: top right;
		transform: skewY(2deg);
		box-shadow: 0 0 200px rgba(0, 0, 0, .3);
	}

	.download .container{ padding-top: .6vw; }

	.download table{ color: #fff; }

	.download td{
		width: 33.33333%;
		padding-right: 15px;
		padding-left: 15px;
		line-height: 2.6;
	}

	.download td:not(:first-child){ text-align: center; }

	.download td:not(:empty){ border: 1px solid #444; }

	.download table .link{ text-transform: uppercase; }
</style>
