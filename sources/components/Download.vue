<template>
    <section class="download"></section>
</template>

<script>
    export default {
        name: 'download'
    }
</script>

<style>
    .download{
        position: relative;
        overflow: visible;
        min-height: 500px;
        background-color: #0f1316;
    }

    .download::before{
        content: '';
        position: absolute;
        top: 0; right: 0; bottom: 0; left: 0;
        background-color: inherit;
        transform-origin: 0;
        transform: skewY(-2deg);
    }
</style>
