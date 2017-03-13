<template>
    <section class="introduction">
        <div class="introduction__container">
            <header class="introduction__header">
                <h1 class="introduction__title">RealEase</h1>
                <h2 class="introduction__subtitle">Never miss another release</h2>

                <a href="https://github.com/alexisfontaine/starred/releases/download/v0.4.1/RealEase-x64.exe" class="button" download>
                    <svg class="icon--windows" viewBox="0 0 50 50"><path d="M1.589 23.55l-0.017-15.31 18.839-2.558v17.868zM23.55 5.225l25.112-3.654v21.979h-25.112zM48.669 26.69l-0.006 21.979-25.112-3.533v-18.446zM20.41 44.736l-18.824-2.58-0.001-15.466h18.825z"></path></svg>
                    Download (v0.4.1)
                </a>
            </header>

            <section class="introduction__content">
                <Release v-for="repository in repositories" :value="repository"></Release>
            </section>
        </div>
    </section>
</template>

<script>
    import Release from './Release'


    export default {
        name: 'introduction',
        data () {
            return { repositories: REPOSITORIES }
        },
        components: { Release }
    }
</script>

<style>
    .introduction{
        --introduction-height: 95vh;

        position: sticky;
        top: 0;
        height: var(--introduction-height);
        background-image: linear-gradient(to bottom, #0e1114 10%, #213042 100%);
        overflow: hidden;
    }

    .introduction::after{
        content: '';
        position: absolute;
        top: 0; right: 0; bottom: 0; left: 0;
        background-image: linear-gradient(to bottom, transparent 70%, rgba(0, 0, 0, .2));
        pointer-events: none;
    }

    .introduction__container{
        display: flex;
        width: 100%;
        max-width: 1600px;
    }

    .introduction__header{
        width: 100%;
        margin-left: 10%;
        margin-top: 10%;
        padding-right: 15px;
        padding-left: 15px;
        z-index: 1;
    }

    .introduction__title{
        margin-top: 0;
        margin-bottom: 25px;
        font-family: 'Muli', sans-serif;
        font-weight: 200;
        font-size: 5.5rem;
        letter-spacing: .5rem;
        color: #d5f9f3;
    }

    .introduction__subtitle{
        margin-bottom: 40px;
        font-family: 'Muli', sans-serif;
        font-weight: 300;
        font-size: 3rem;
    }

    .introduction__content{
        position: relative;
        width: 100%; height: var(--introduction-height);
        margin-right: 8%;
        padding-top: 5%;
        padding-right: 20px;
        perspective: 1000px;
    }

    .introduction__content::before{
        content: '';
        position: absolute;
        top: 0; right: 0; bottom: 15%;
        width: 5px;
        background-image: linear-gradient(to bottom, #55c1b1 40%, transparent 100%);
        box-shadow: 0 -10rem 10px rgba(0, 0, 0, .1);
    }

    .introduction__header .button{
        display: inline-flex;
        align-items: center;
    }

    .introduction__header .button:hover .icon{
        fill: #fff;
    }

    .icon--windows{
        width: 25px; height: 35px;
        margin-right: 20px;
        fill: #eee;
    }

    @media screen and (max-width: 1100px){
        .introduction__content{ margin-right: 5%; }
    }
</style>
