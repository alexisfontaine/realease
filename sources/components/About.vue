<template>
	<footer class="footer">
		Made with <i class="heart"></i> by
		<a href="https://github.com/alexisfontaine" class="author" target="_blank">Alexis</a>
	</footer>
</template>

<script>
	export default {
		name: 'about'
	}
</script>

<style lang="scss">
	@import 'colors';


	.footer{
		position: relative;
		line-height: 7rem;
		font-size: 1.6rem;
		text-align: center;
		background-color: inherit;
	}

	.heart{
		display: inline-block;
		margin-right: 3px;
		margin-left: 3px;
		font-size: 1.8rem;
		color: $red;
		font-style: normal;
		line-height: 1;
		animation: heartbeat 2s infinite;
	}

	.heart::before{ content: '❤'; }

	.author{
		color: $highlight-color;
		font-weight: 600;
		text-decoration: none;
		transition: color .2s;
	}

	.author:hover{ color: $highlight-color-hover; }
</style>
