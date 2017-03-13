<template>
    <main>
        <Introduction></Introduction>
        <Download></Download>
    </main>
</template>

<script>
    import Introduction from './Introduction'
    import Download from './Download'


    export default {
        name: 'application',
        components: {
            Introduction,
            Download
        }
    }
</script>
