<template>
    <article class="release">
        <Repository></Repository>

        <section class="release__content">
            <div class="release__content__container">
                <h3 class="release__title">
                    <svg viewBox="0 0 14 16" class="icon icon--release">
                        <path d="M7.73 1.73C7.26 1.26 6.62 1 5.96 1H3.5C2.13 1 1 2.13 1 3.5v2.47c0 .66.27 1.3.73 1.77l6.06 6.06c.39.39 1.02.39 1.41 0l4.59-4.59a.996.996 0 0 0 0-1.41L7.73 1.73zM2.38 7.09c-.31-.3-.47-.7-.47-1.13V3.5c0-.88.72-1.59 1.59-1.59h2.47c.42 0 .83.16 1.13.47l6.14 6.13-4.73 4.73-6.13-6.15zM3.01 3h2v2H3V3h.01z"></path>
                    </svg>
                    <a href="https://github.com/facebook/flow/releases/tag/v0.41.0" target="_blank">v0.41.0</a>
                </h3>

                <p>Notable bug fixes:</p>
                <ul>
                    <li>Lots of improvements to typing React components (which may uncover previously missed errors). This release primarily focused on these tweaks to React typing</li>
                    <li>Previously Flow did not track much about mixins from <code>React.createClass</code> components (including mixed in proptypes). This has been fix and may uncover new proptypes errors in createClass components</li>
                    <li>The internal React libdef for <code>findDOMNode</code> has been updated to include <code>null</code> in its return-type now (since this function can, indeed, return <code>null</code>)</li>
                </ul>

                <p>Misc:</p>
                <ul>
                    <li><code>flow get-importers</code> has been removed. It was rarely used and added an incredible amount of complexity to the codebase</li>
                    <li>Lots of updates to builtin flow libs</li>
                </ul>
            </div>
        </section>
    </article>
</template>

<script>
    import Repository from './Repository'


    export default {
        name: 'application',
        components: { Repository }
    }
</script>

<style>
    .release{
        display: flex;
        flex-direction: column;
        height: 50%;
        max-height: 400px;
        margin-bottom: 20px;
        margin-right: 20px;
        padding: 20px;
        background-color: #fff;
        border-radius: 2px;
        font-size: 1.4rem;
        color: #444;
        box-shadow: 0 5px 10px rgba(0, 0, 0, .2);
        animation-timing-function: ease-out;
    }

    .release.add{
        animation-name: flip;
        animation-duration: .8s;
    }

    .release.hide{
        animation-name: disappear;
        animation-delay: .6s;
        animation-duration: .4s;
        animation-fill-mode: forwards;
    }

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
        margin-top: 0;
        margin-bottom: 0;
        font-size: 2rem;
        font-weight: 600;
        line-height: 2;
    }

    .release__content{
        position: relative;
        height: 100%;
        margin-top: 10px;
        background-color: var(--gray);
        overflow-y: overlay;
    }

    .release__content::after{
        content: '';
        position: absolute;
        top: 0; left: 0;
        width: 100%; height: 100%;
        box-shadow: inset 10px 0 12px 4px var(--gray);
        pointer-events: none;
    }

    .release__content h1, .release__content h2, .release__content h3, .release__content h4{ font-weight: 500; }

    .release__content img{
        display: block;
        max-width: 100%;
        margin: 15px auto;
    }

    .release__content img[align=right]{ margin-left: 15px; }
    .release__content img[align=left]{ margin-right: 15px; }

    .release__content__container{
        height: 100%;
        padding: 20px;
        overflow-y: overlay;
    }


    .release ::-webkit-scrollbar{
        width: var(--sidebar-size);
        height: var(--sidebar-size);
    }

    .release ::-webkit-scrollbar-corner, .release ::-webkit-scrollbar-track{ background-color: transparent; }

    .release ::-webkit-scrollbar-track:hover{ background-color: rgba(0, 0, 0, .1); }

    .release ::-webkit-scrollbar-track:active{ background-color: rgba(0, 0, 0, .15); }

    .release ::-webkit-scrollbar-thumb{
        min-width: 10rem;
        min-height: 10rem;
        background-color: rgba(0, 0, 0, .2);
        background-clip: padding-box;
        border: 5px solid transparent;
        border-radius: 50px;
    }

    .release ::-webkit-scrollbar-thumb:active{ background-color: rgba(0, 0, 0, .35); }

    .release ::-webkit-scrollbar-thumb:active, .release ::-webkit-scrollbar-thumb:hover{ border-width: 4px; }

    .release ::-webkit-scrollbar-thumb:window-inactive{ background-color: rgba(0, 0, 0, .1); }
</style>
