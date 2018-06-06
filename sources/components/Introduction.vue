<template>
	<section class="introduction">
		<div id="introduction" class="introduction__container">
			<header class="introduction__header">
				<h1 class="introduction__title">RealEase</h1>
				<h2 class="introduction__subtitle">Never miss another release</h2>

				<a id="download_windows64" :href="`https://realease.herokuapp.com/download/${version}/RealEase-x64.exe`" class="button" :title="`Windows 64-bit ${version} (.exe)`" download>
					<svg class="icon--os--windows" viewBox="0 0 50 50">
						<path d="M1.589 23.55l-0.017-15.31 18.839-2.558v17.868zM23.55 5.225l25.112-3.654v21.979h-25.112zM48.669 26.69l-0.006 21.979-25.112-3.533v-18.446zM20.41 44.736l-18.824-2.58-0.001-15.466h18.825z"></path>
					</svg>
					Download
				</a>
				<a id="download_windows32" :href="`https://realease.herokuapp.com/download/${version}/RealEase-x86.exe`" class="button" :title="`Windows 32-bit ${version} (.exe)`" download>
					<svg class="icon--os--windows" viewBox="0 0 50 50">
						<path d="M1.589 23.55l-0.017-15.31 18.839-2.558v17.868zM23.55 5.225l25.112-3.654v21.979h-25.112zM48.669 26.69l-0.006 21.979-25.112-3.533v-18.446zM20.41 44.736l-18.824-2.58-0.001-15.466h18.825z"></path>
					</svg>
					Download
				</a>
				<a id="download_macos" :href="`https://realease.herokuapp.com/download/${version}/RealEase-x64.dmg`" class="button" :title="`macOS 64-bit ${version} (.dmg)`" download>
					<svg class="icon--os--macos" viewBox="0 0 50 50">
						<path d="M39.054 34.065q-1.093 3.504-3.448 7.009-3.617 5.495-7.205 5.495-1.374 0-3.925-0.897-2.411-0.897-4.233-0.897-1.71 0-3.981 0.925-2.271 0.953-3.701 0.953-4.261 0-8.439-7.261-4.121-7.317-4.121-14.102 0-6.392 3.168-10.485 3.14-4.037 7.962-4.037 2.019 0 4.962 0.841 2.916 0.841 3.869 0.841 1.262 0 4.009-0.953 2.86-0.953 4.85-0.953 3.336 0 5.972 1.822 1.458 1.009 2.916 2.804-2.215 1.878-3.196 3.308-1.822 2.635-1.822 5.803 0 3.476 1.934 6.252t4.43 3.533zM28.512 1.179q0 1.71-0.813 3.813-0.841 2.103-2.607 3.869-1.514 1.514-3.028 2.019-1.037 0.308-2.916 0.477 0.084-4.177 2.187-7.205 2.075-3 7.009-4.149 0.028 0.084 0.070 0.308t0.070 0.308q0 0.112 0.014 0.28t0.014 0.28z"></path>
					</svg>
					Download
				</a>
				<a id="download_other" href="#downloads" class="button">Download</a>
				<a id="download" href="#downloads" class="link">Other platforms</a>
			</header>

			<section class="introduction__content">
				<div id="releases" class="releases">
					<Release v-for="repository in repositories" :key="repository.resourcePath" :value="repository"></Release>
				</div>
			</section>
		</div>
	</section>
</template>

<script>
	import Release from './Release'


	export default {
		name: 'introduction',
		data: () => ({ repositories: REPOSITORIES, version: VERSION.tag }),
		components: { Release }
	}
</script>

<style lang="scss">
	@import 'colors.scss';


	$introduction-height: 95vh;

	.introduction{
		position: sticky;
		top: 0;
		height: $introduction-height;
		background-image: linear-gradient(to bottom, #0e1114 10%, #213042 100%);
		overflow: hidden;
	}

	.introduction::after{
		content: '';
		position: absolute;
		top: 0; right: 0; bottom: 0; left: 0;
		background-image: linear-gradient(to bottom, transparent 70%, rgba(0, 0, 0, .1));
		pointer-events: none;
	}

	.introduction__container{
		display: flex;
		width: 100%;
		max-width: 1800px;
		margin-right: auto;
		margin-left: auto;
	}

	.introduction__header{
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		align-items: flex-start;
		margin-left: 10%;
		padding-right: 1.5rem;
		padding-left: 1.5rem;
		color: #91b2b3;
	}

	.introduction__header .link{ line-height: 3rem; }

	.introduction__title{
		margin-top: 20vh;
		margin-bottom: .5rem;
		font-family: 'Muli', sans-serif;
		font-weight: 200;
		font-size: 5.5rem;
		letter-spacing: .5rem;
		color: $lighter-green;
	}

	.introduction__subtitle{
		margin-top: 0;
		margin-bottom: 4rem;
		font-family: 'Muli', sans-serif;
		font-weight: 300;
		font-size: 3rem;
	}

	.introduction__content{
		position: relative;
		width: 50%; height: $introduction-height;
		margin-right: 8%;
		margin-left: 3rem;
		padding-right: 2rem;
	}

	.introduction__content::before{
		content: '';
		position: absolute;
		top: 0; right: 0; bottom: 15%;
		width: 5px;
		background-image: linear-gradient(to bottom, #55c1b1 40%, transparent 100%);
		box-shadow: 0 -10rem 10px rgba(0, 0, 0, .1);
	}

	.releases{
		position: relative;
		top: 10%;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		perspective: 1000px;
		animation-duration: .6s;
		animation-fill-mode: both;
		animation-timing-function: ease-out;
	}

	.releases.translate{ animation-name: translate; }

	.introduction__header .button{
		display: none;
		justify-content: center;
		align-items: center;
		font-size: 1.8rem;

		&.active{ display: inline-flex; }
		&:hover .icon--os{ fill: #fff; }
	}

	[class^=icon--os--]{
		height: 3.5rem;
		margin-left: -1rem;
		fill: #eee;
		transition: fill .2s;
	}

	.icon--os--windows{
		width: 2.5rem;
		margin-right: 1.9rem;
	}

	.icon--os--macos{
		width: 3.2rem;
		margin-right: 1.2rem;
	}

	@media screen and (max-width: 1100px){
		.introduction__content{ margin-right: 5%; }
	}
</style>
