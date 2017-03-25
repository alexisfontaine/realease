<template>
    <main>
        <Introduction/>
        <Download/>
        <About/>
    </main>
</template>

<script>
    import Introduction from './Introduction'
    import Download from './Download'
    import About from './About'


    export default {
        name: 'application',
        components: {
            Introduction,
            Download,
            About
        }
    }
</script>
