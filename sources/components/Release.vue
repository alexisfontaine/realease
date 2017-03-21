<template>
    <article class="release">
        <Repository :value="value"></Repository>

        <section class="release__content">
            <div class="release__content__container" v-if="value.release">
                <h3 class="release__title">
                    <svg viewBox="0 0 14 16" class="icon icon--release">
                        <path d="M7.73 1.73C7.26 1.26 6.62 1 5.96 1H3.5C2.13 1 1 2.13 1 3.5v2.47c0 .66.27 1.3.73 1.77l6.06 6.06c.39.39 1.02.39 1.41 0l4.59-4.59a.996.996 0 0 0 0-1.41L7.73 1.73zM2.38 7.09c-.31-.3-.47-.7-.47-1.13V3.5c0-.88.72-1.59 1.59-1.59h2.47c.42 0 .83.16 1.13.47l6.14 6.13-4.73 4.73-6.13-6.15zM3.01 3h2v2H3V3h.01z"></path>
                    </svg>
                    <a :href="`https://github.com${value.path}/releases/tag/${value.release.tag}`" target="_blank">
                        {{ value.release.name || value.release.tag }}
                    </a>
                </h3>

                <div v-html="value.release.description"></div>
            </div>
        </section>
    </article>
</template>

<script>
    import Repository from './Repository'


    export default {
        name: 'application',
        components: { Repository },
        props: {
            value: {
                type: Object,
                required: true
            }
        }
    }
</script>

<style>
    .release{
        display: flex;
        flex-direction: column;
        width: 100%; height: 50%;
        max-width: 550px;
        min-height: 250px;
        max-height: 400px;
        margin-bottom: 20px;
        margin-right: 20px;
        margin-left: 20px;
        padding: 20px;
        background-color: #fff;
        border-radius: 2px;
        box-shadow: 0 5px 10px rgba(0, 0, 0, .2);
        animation-duration: .6s;
        animation-timing-function: ease-out;
    }

    .release.flip{ animation-name: flip; }

    .release, .release::before{
        transform-style: preserve-3d;
        backface-visibility: hidden;
    }

    .release::before{
        content: '';
        position: absolute;
        top: 0; right: 0; bottom: 0; left: 0;
        background-color: #fff;
        transform: rotateX(180deg);
        z-index: 1;
    }

    .release__title{
        margin-top: 0 !important;
        margin-bottom: 1rem !important;
        line-height: 1.4;
    }

    .release__content{
        position: relative;
        display: inherit;
        margin-top: 10px;
        font-size: 1.3rem;
        background-color: var(--gray);
    }

    .release__content::after{
        content: '';
        position: absolute;
        top: 0; left: 0; right: 0; bottom: 0;
        box-shadow: inset 10px 0 12px 4px var(--gray);
        pointer-events: none;
    }

    .release__content h1, .release__content h2{
        margin-top: 1.8rem;
        margin-bottom: 1.8rem;
        font-size: 1.7rem;
    }

    .release__content h2,
    .release__content h3,
    .release__content h5{ font-weight: 600; }

    .release__content h3, .release__content h4{ font-size: 1.6rem; }

    .release__content h3{
        margin-top: 1.4rem;
        margin-bottom: 1.4rem;
    }

    .release__content h4, .release__content h6{ font-weight: 500; }

    .release__content h4{
        margin-top: 1.2rem;
        margin-bottom: 1.2rem;
    }

    .release__content h5, .release__content h6{
        margin-top: 1rem;
        margin-bottom: 1rem;
        font-size: 1.4rem;
    }

    .release__content ul{ padding-left: 2.2rem; }

    .release__content img{
        display: block;
        max-width: 100%;
        margin: 1.5rem auto;
    }

    .release__content img[align=right]{ margin-left: 1.5rem; }
    .release__content img[align=left]{ margin-right: 1.5rem; }

    .release__content__container{
        padding: 2rem;
        overflow-x: hidden;
        overflow-y: overlay;
    }

    .release__content__container > div{
        padding-top: 1rem;
        border-top: 1px dashed #ddd;
    }

    .release__content__container > div > :first-child{ margin-top: 0; }

    .release__content__container > div > :last-child{ margin-bottom: 0; }

    .release ::-webkit-scrollbar{
        width: var(--scrollbar-width);
        height: var(--scrollbar-width);
    }

    .release ::-webkit-scrollbar-corner, .release ::-webkit-scrollbar-track{ background-color: transparent; }

    .release ::-webkit-scrollbar-track:hover{ background-color: rgba(0, 0, 0, .1); }

    .release ::-webkit-scrollbar-track:active{ background-color: rgba(0, 0, 0, .15); }

    .release ::-webkit-scrollbar-thumb{
        min-width: var(--scrollbar-length);
        min-height: var(--scrollbar-length);
        background-color: rgba(0, 0, 0, .2);
        background-clip: padding-box;
        border: 5px solid transparent;
        border-radius: 50px;
    }

    .release ::-webkit-scrollbar-thumb:active{ background-color: rgba(0, 0, 0, .35); }

    .release ::-webkit-scrollbar-thumb:active, .release ::-webkit-scrollbar-thumb:hover{ border-width: 4px; }

    .release ::-webkit-scrollbar-thumb:window-inactive{ background-color: rgba(0, 0, 0, .1); }

    .icon.icon--release{
        height: 24px;
        margin-right: 4px;
    }
</style>
